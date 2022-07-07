use crate::conf::config::{AppConfig, Strategy};
use crate::eth::Wallet;
use crate::eth::{self, checksum};
use crate::utils;
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
                    return wallet;
                }
            }
            Strategy::Startswith => {
                if address.starts_with(&config.pattern) {
                    return wallet;
                }
            }
        }

        processed.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn spawn_threads(
    count: u32,
    config: &AppConfig,
    tx: &Sender<Wallet>,
    found: &Arc<AtomicBool>,
    processed: &Arc<AtomicU64>,
) -> Vec<thread::JoinHandle<Result<(), mpsc::SendError<Wallet>>>> {
    println!("Starting generation with {} threads.", count);
    let mut threads = vec![];

    for _ in 0..count {
        let thread_tx = tx.clone();
        let config_clone = config.clone();
        let found_clone = found.clone();
        let processed_clone = processed.clone();

        threads.push(thread::spawn(move || {
            thread_tx.send(find_address_starting_with(
                found_clone,
                processed_clone,
                config_clone,
            ))
        }))
    }

    threads
}

pub fn run(count: u32, config: AppConfig) {
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

    let threads = spawn_threads(count, &config, &tx, &found, &processed);

    let start_time = Instant::now();
    let mut last_generated = 0;
    loop {
        if let Ok(wallet) = rx.recv_timeout(Duration::from_millis(1000)) {
            println!("Private key: {}", wallet.private_key);
            println!("Address: 0x{}", checksum(&wallet.public_key));
            break;
        }

        let elapsed = start_time.elapsed().as_secs();
        let generated = processed.load(Ordering::Relaxed);
        let speed = generated - last_generated;
        last_generated = generated;

        let difficulty = utils::calculate_difficulty(&config.pattern, config.casesensitive);
        let estimated_time: u64 = utils::calculate_estimated_time(speed, difficulty);

        let time_left = utils::time_left(estimated_time, elapsed);

        println!(
            "Speed: {} h/s. Up-time: {}s. Max time left: {}s. Generated {} addresses",
            speed, elapsed, time_left, generated
        );
    }

    found.store(true, Ordering::Relaxed);

    for t in threads {
        _ = t.join();
    }
}
