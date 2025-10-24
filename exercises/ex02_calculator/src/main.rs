use std::io::{self, Write};

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            match input.trim().parse::<f64>() {
                Ok(num) => return num,
                Err(_) => println!("Invalid number. Please try again."),
            }
        } else {
            println!("Failed to read input. Try again.");
        }
    }
}

fn read_operator() -> char {
    loop {
        print!("Enter an operator (+, -, *, /): ");
        io::stdout().flush().unwrap();

        let mut op = String::new();
        if io::stdin().read_line(&mut op).is_ok() {
            match op.trim() {
                "+" | "-" | "*" | "/" => return op.trim().chars().next().unwrap(),
                _ => println!("Invalid operator. Try again."),
            }
        } else {
            println!("Failed to read input. Try again.");
        }
    }
}

fn calculate(num1: f64, num2: f64, op: char) -> Option<f64> {
    match op {
        '+' => Some(num1 + num2),
        '-' => Some(num1 - num2),
        '*' => Some(num1 * num2),
        '/' => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                println!("Error: Division by zero.");
                None
            }
        }
        _ => None,
    }
}

fn main() {
    let num1 = read_number("Enter the first number: ");
    let num2 = read_number("Enter the second number: ");
    let operator = read_operator();

    match calculate(num1, num2, operator) {
        Some(result) => println!("Result: {}", result),
        None => println!("Calculation failed."),
    }
}