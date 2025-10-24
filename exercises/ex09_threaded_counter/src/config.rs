use crate::mutex_type::MutexType;

pub struct Config {
    pub number_of_threads: usize,
    pub mutex_type: MutexType,
    pub increment_count: usize
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let number_of_threads: usize = args
            .next()
            .ok_or("Didn't get a number of threads argument")?
            .parse()
            .map_err(|_| "number_of_threads must be an integer")?;

        let mutex_type: MutexType = args
            .next()
            .ok_or("Didn't get a mutex type")?
            .parse()?;

        let mut increment_count = 1;
        if let Some(s) = args.next() {
            increment_count = s.parse::<usize>()
                .map_err(|_| "increment_count must be an integer")?;
        }
            
        Ok(Config {
            number_of_threads,
            mutex_type,
            increment_count
        })
    }
}