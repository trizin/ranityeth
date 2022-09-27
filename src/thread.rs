use crate::conf::config::{AppConfig, Strategy};
use crate::eth::Wallet;
use crate::eth::{self, checksum};
use crate::fs::append_to_file;
use crate::utils;
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
    loop {
        if found.load(Ordering::Relaxed) {
            return Wallet::new();
        }
        let wallet = Wallet::new();

        let address = match config.contract {
            false => wallet.public_key.clone(),
            true => eth::generate_contract_address(&wallet),
        };

        let address = match config.casesensitive {
            false => address,
            true => checksum(&address),
        };

        match config.strategy {
            Strategy::Contains => {
                if address.contains(&config.pattern) {
                    found.store(true, Ordering::Relaxed);

                    if !config.continuous {
                        return wallet;
                    } else {
                        _ = append_to_file("./pks", wallet.private_key.as_str());
                    }
                }
            }
            Strategy::Startswith => {
                let mut score = 0;
                // find how many characters match at the beginning
                for (i, c) in config.pattern.chars().enumerate() {
                    if address.chars().nth(i).unwrap() == c {
                        score += 1;
                    }
                }

                if score > best_score.load(Ordering::Relaxed) {
                    println!("SCORE: {}", score);
                    best_score.store(score, Ordering::Relaxed);

                    if score == config.pattern.len() as u64 {
                        found.store(true, Ordering::Relaxed);
                        if !config.continuous {
                            return wallet;
                        } else {
                            _ = append_to_file("./pks", wallet.private_key.as_str());
                        }
                    }
                    write_wallet_info(&wallet, &config)
                }
            }
            Strategy::Trailing => {
                // config.pattern should be one character
                // count how many consecutive characters are at the beginning of the address

                let mut count = 0;
                for c in address.chars() {
                    if c == config.pattern.chars().next().unwrap() {
                        count += 1;
                    } else {
                        break;
                    }
                }
                if count > best_score.load(Ordering::Relaxed) {
                    println!("SCORE: {}", count);
                    best_score.store(count, Ordering::Relaxed);
                    write_wallet_info(&wallet, &config)
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

pub fn write_wallet_info(wallet: &Wallet, config: &AppConfig) {
    println!("Private key: {}", wallet.private_key);
    println!("Address: 0x{}", checksum(&wallet.public_key));
    if config.contract {
        let contract_address = eth::generate_contract_address(&wallet);
        println!("Contract address: 0x{}", checksum(&contract_address));
    }
    print!("--------------\n\n")
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
        if let Ok(wallet) = rx.recv_timeout(Duration::from_millis(1000)) {
            write_wallet_info(&wallet, &config);
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
            std::io::stdout().flush();
        }
    }

    for t in threads {
        _ = t.join();
    }
}
