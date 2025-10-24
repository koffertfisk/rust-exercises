use std::{
    cmp::Ordering,
    io::{self, Write},
};

use rand::Rng;

enum InputResult {
    Number(i32),
    Quit,
}

fn read_input(prompt: &str) -> InputResult {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            match trimmed.to_lowercase().as_str() {
                "q" | "quit" => return InputResult::Quit,
                _ => match trimmed.parse::<i32>() {
                    Ok(num) => return InputResult::Number(num),
                    Err(_) => println!("Invalid input. Please enter a number or 'q' to quit."),
                },
            }
        } else {
            println!("Failed to read input. Try again.");
        }
    }
}

fn main() {
    println!("Guess the number! (Type 'q' or 'quit' to exit)");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        match read_input("Please input your guess: ") {
            InputResult::Quit => {
                println!("Goodbye!");
                break;
            }
            InputResult::Number(guess) => {
                println!("You guessed: {}", guess);

                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
            }
        }
    }

    println!("The secret number was {}", secret_number);
}