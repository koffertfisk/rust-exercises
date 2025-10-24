use std::str::FromStr;

pub enum MutexType {
    ArcMutex,
    RwLock
}

impl FromStr for MutexType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "arcmutex" | "arc" | "mutex" => Ok(MutexType::ArcMutex),
            "rwlock" | "rw" => Ok(MutexType::RwLock),
            _ => Err("Unknown mutex type (use 'ArcMutex' or 'RWLock')"),
        }
    }
}