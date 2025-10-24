use std::env;

pub struct Config {
    pub file_path: String,
    pub number_of_sentences: usize,
    pub number_of_threads: usize
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let mut number_of_sentences: usize = 1;
        if let Some(s) = args.next() {
            number_of_sentences = s.parse::<usize>()
                .map_err(|_| "number_of_sentences must be an integer")?;
        }

        let mut number_of_threads: usize = 1;
        if let Some(s) = args.next() {
            number_of_threads = s.parse::<usize>()
                .map_err(|_| "number_of_threads must be an integer")?;
        }

        Ok(Config {
            file_path,
            number_of_sentences,
            number_of_threads
        })
    }

}