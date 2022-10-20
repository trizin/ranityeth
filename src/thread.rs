use crate::conf::config::AppConfig;
use crate::eth::Wallet;
use crate::eth::{self, checksum};
use crate::fs::append_to_file;
use crate::strategy::{Score, Strategy};
use crate::{create2, utils};
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

pub fn find_address_starting_with(
    found: Arc<AtomicBool>,
    processed: Arc<AtomicU64>,
    config: AppConfig,
    best_score: Arc<AtomicU64>,
) -> Wallet {
    let mut salt = create2::generate_salt(); // used for create2
    let mut wallet = Wallet::new();
    let strategy = &config.strategy;
    let bytecode_hash = create2::bytecode_keccak(&config.bytecode);
    loop {
        if found.load(Ordering::Relaxed) {
            return Wallet::new();
        }

        let mut address;

        if config.create2 {
            salt = create2::derive_salt(salt);
            address = create2::calc_addr(config.deployer.as_str(), salt, bytecode_hash);
        } else {
            wallet = Wallet::new();
            address = match config.contract {
                false => wallet.public_key.clone(),
                true => eth::generate_contract_address(&wallet),
            };
        }

        address = match config.casesensitive {
            false => address,
            true => checksum(&address),
        };

        let _score = strategy.score(&config, &address);
        match strategy {
            Strategy::Contains => {
                if _score == 1 {
                    if !config.continuous {
                        write_wallet_info(&wallet, &config, salt, _score);
                        found.store(true, Ordering::Relaxed);
                        return wallet;
                    } else {
                        _ = append_to_file("./pks", format!("{}\n", wallet.private_key).as_str());
                    }
                }
            }
            Strategy::Startswith => {
                if _score > best_score.load(Ordering::Relaxed) || config.continuous {
                    if !config.continuous {
                        best_score.store(_score, Ordering::Relaxed);
                        write_wallet_info(&wallet, &config, salt, _score);
                    }

                    if _score == config.pattern.len() as u64 {
                        if !config.continuous {
                            found.store(true, Ordering::Relaxed);
                            return wallet;
                        } else {
                            _ = append_to_file(
                                "./pks",
                                format!("{}\n", wallet.private_key).as_str(),
                            );
                        }
                    }
                }
            }
            Strategy::Trailing => {
                // config.pattern should be one character
                // count how many consecutive characters are at the beginning of the address
                if _score > best_score.load(Ordering::Relaxed) {
                    best_score.store(_score, Ordering::Relaxed);
                    write_wallet_info(&wallet, &config, salt, _score)
                }
            }
        }

        processed.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn spawn_threads(
    config: &AppConfig,
    tx: &Sender<Wallet>,
    found: &Arc<AtomicBool>,
    processed: &Arc<AtomicU64>,
) -> Vec<thread::JoinHandle<Result<(), mpsc::SendError<Wallet>>>> {
    println!("Starting generation with {} threads.", config.threads);
    let mut threads = vec![];
    let best_score = Arc::new(AtomicU64::new(0));

    for _ in 0..config.threads {
        let thread_tx = tx.clone();
        let config_clone = config.clone();
        let found_clone = found.clone();
        let processed_clone = processed.clone();
        let best_score_clone = best_score.clone();

        threads.push(thread::spawn(move || {
            thread_tx.send(find_address_starting_with(
                found_clone,
                processed_clone,
                config_clone,
                best_score_clone,
            ))
        }))
    }

    threads
}

pub fn write_wallet_info(wallet: &Wallet, config: &AppConfig, salt: [u8; 32], score: u64) {
    print!("--------------\n");
    println!("SCORE: {}", score);
    if config.create2 {
        let salt_hex = hex::encode(&salt);
        println!("Found salt: {}", salt_hex);
    } else {
        println!("Private key: {}", wallet.private_key);
        println!("Address: 0x{}", checksum(&wallet.public_key));
    }
    if config.contract {
        let contract_address;
        if config.create2 {
            contract_address = create2::calc_addr(
                config.deployer.as_str(),
                salt,
                create2::bytecode_keccak(&config.bytecode),
            );
        } else {
            contract_address = eth::generate_contract_address(&wallet);
        }
        println!("Contract address: 0x{}", checksum(&contract_address));
    }
    print!("--------------\n\n\n");
}

pub fn run(config: AppConfig) {
    let mut privatkey_list: Vec<String> = Vec::new();
    for _ in 0..10000 {
        let acc = eth::Wallet::new();
        privatkey_list.push(acc.private_key);
    }

    if !utils::is_possible_pattern(&config.pattern) {
        println!("Impossible pattern. Use 0-9, a-f");
        return;
    }

    let (tx, rx) = mpsc::channel();
    let found = Arc::new(AtomicBool::new(false));
    let processed = Arc::new(AtomicU64::new(0));

    let threads = spawn_threads(&config, &tx, &found, &processed);

    let start_time = Instant::now();
    let mut last_generated = 0;
    loop {
        if let Ok(_wallet) = rx.recv_timeout(Duration::from_millis(1000)) {
            if found.load(Ordering::Relaxed) {
                break;
            }
        }

        let elapsed = start_time.elapsed().as_secs();
        let generated = processed.load(Ordering::Relaxed);
        let speed = generated - last_generated;
        last_generated = generated;

        let difficulty = utils::calculate_difficulty(&config.pattern, config.casesensitive);
        let estimated_time: u64 = utils::calculate_estimated_time(speed, difficulty);

        let time_left = utils::time_left(estimated_time, elapsed);

        if config.strategy != Strategy::Trailing {
            print!(
                "\r Speed: {} h/s. Up-time: {}s. Max time left: {}s. Generated {} addresses",
                speed, elapsed, time_left, generated
            );
            _ = std::io::stdout().flush();
        }
    }

    for t in threads {
        _ = t.join();
    }
}
