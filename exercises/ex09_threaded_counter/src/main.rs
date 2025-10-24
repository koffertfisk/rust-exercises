mod mutex_type;
mod config;

use std::env;
use std::process;
use std::error::Error;
use std::sync::RwLock;

use config::Config;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::mutex_type::MutexType;

fn main() {
    println!("Multi Threaded Counter\n");
    
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.mutex_type {
        MutexType::ArcMutex => {
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];

            for _ in 0..config.number_of_threads {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();

                    *num += config.increment_count;
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("Result: {}", *counter.lock().unwrap());
        }
        MutexType::RwLock => { 
            let counter = Arc::new(RwLock::new(0));
            let mut handles = vec![];

            for _ in 0..config.number_of_threads {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut num = counter.write().expect("poisoned");
                    *num += config.increment_count;
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("Result: {}", *counter.read().unwrap());
        }
    }

    Ok(())
}