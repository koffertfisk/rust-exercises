use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub email: String,
}

impl User {
    pub fn from_json_string(data: &str) -> Result<User, &'static str> {
        let u: User = serde_json::from_str(data).unwrap();
        Ok(u)
    }

    pub fn from_json_file(file_path: &str) -> Result<User, &'static str> {
        let data = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let u: User = serde_json::from_str(&data).unwrap();
        Ok(u)
    }
}