use std::fs::read_to_string;

use crate_exploration_and_json_deserialization::{calculate_average_age, User};
use serde_json::from_str;

fn main() {
    let json = read_to_string("data.json").expect("failed to read data.json");
    let users: Vec<User> = from_str(&json).expect("invalid JSON for Vec<User>");
    println!("{:#?}", users);
    println!("Average age: {}", calculate_average_age(&users));
}
