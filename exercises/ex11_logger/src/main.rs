mod config;

use std::error::Error;
use std::sync::mpsc::{channel};
use std::{env, process, thread};

use chrono::{DateTime, Utc};
use config::Config;
use file_locking_and_appending_to_a_log::append_log_entry;
use file_locking_and_appending_to_a_log::{random_sentences};

fn main() {
    println!("Random Sentence File Logger\n");

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
    let (tx, rx) = channel();
    for _ in 0..config.number_of_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let thread_id = thread::current().id();
            
            for line in random_sentences(config.number_of_sentences) {
                let utc: DateTime<Utc> = Utc::now();
                let message = format!("{utc} [Thread {:?}] - {line}", thread_id);
                println!("{message}");
                if tx.send(message).is_err() {
                    eprintln!("Receiver dropped, exiting thread.");
                    break;
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    }

    drop(tx);

    for message in rx {
        let result = append_log_entry(&config.file_path, &message);
        match result {
            Ok(()) => println!("Entry successfully written to log file"),
            Err(_) => println!("Error writing to log file")
        } 
    }
    
    Ok(())
}