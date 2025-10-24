mod config;

use std::env;
use std::fs;
use std::process;
use std::error::Error;

use config::Config;

use error_handling_file_reader::count_occurrences;
use error_handling_file_reader::count_occurrences_insensitive;
use error_handling_file_reader::search;
use error_handling_file_reader::search_case_insensitive;

fn main() {
    println!("File Reader\n");
    
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
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in results {
        println!("{line}");
    }

    if config.count_occurrences {
        let occurrences = if config.ignore_case {
            count_occurrences_insensitive(&config.query, &contents)
        } else {
            count_occurrences(&config.query, &contents)
        };

        println!("'{}' occurrences: {}", &config.query, occurrences);
    }

    Ok(())
}