use std::{
    collections::HashMap, io::{self, Write}
};

enum Numeric {
    Int(i32),
    Float(f64)
}

impl Numeric {
    fn to_f64(self) -> f64 {
        match self {
            Numeric::Int(x) => x as f64,
            Numeric::Float(x) => x
        }
    }
}

impl std::fmt::Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numeric::Int(x) => write!(f, "{x}"),
            Numeric::Float(x) => write!(f, "{x}"),
        }
    }
}

enum InputResult {
    Number(i32),
    Done,
    Quit,
}

enum Statistic {
    Sum,
    Average,
    Median,
    Mode
}

fn first_middle_index<T>(v: &[T]) -> Option<usize> {
    match v.len() {
        0 => None,
        n if n % 2 == 0 => Some((n / 2) - 1),
        n => Some(n / 2),
    }
}

fn calculate_median(numbers: &[i32]) -> Numeric {
    let numbers_len = numbers.len();
    
    let mut numbers_sorted: Vec<i32> = numbers.to_vec();
    numbers_sorted.sort();
    
    let middle = first_middle_index(&numbers_sorted);

    match middle {
        Some(index) => {
            if numbers_len % 2 == 0
            {
                let median: f64 = (numbers_sorted[index] as f64 + numbers_sorted[index + 1] as f64) / 2.0;
                Numeric::Float(median)
            } else {
                let median: f64 = numbers_sorted[index] as f64;
                Numeric::Float(median)
            }
        },
        None => Numeric::Int(0)
    }
}

fn calculate_sum(numbers: &[i32]) -> Numeric {
    Numeric::Int(numbers.iter().sum())
}

fn calculate_average(numbers: &[i32]) -> Numeric {
    let sum = calculate_sum(numbers).to_f64();
    
    Numeric::Float(sum / numbers.len() as f64)
}

fn calculate_mode(numbers: &[i32]) -> Numeric {
    if numbers.is_empty() {
        return Numeric::Int(0);
    }

    let mut occurrences = HashMap::new();

    for &num in numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    let (mode, _) = occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    Numeric::Int(mode)
}

fn calculate(numbers: &[i32], statistic: Statistic) -> Numeric {
    match statistic {
        Statistic::Sum => calculate_sum(numbers),
        Statistic::Median => calculate_median(numbers),
        Statistic::Average => calculate_average(numbers),
        Statistic::Mode => calculate_mode(numbers)
    }
}

fn read_input(prompt: &str) -> InputResult {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            match trimmed.to_lowercase().as_str() {
                "" => return InputResult::Done,
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
    println!("Find the Median & Mode");

    let mut should_continue = true;

    loop {
        
        let mut numbers: Vec<i32> = Vec::new();

        loop {
            match read_input("Please input a number or leave blank: ") {
                InputResult::Quit => {
                    println!("Goodbye!");
                    should_continue = false;
                    break;
                }
                InputResult::Done => {
                    break;
                }
                InputResult::Number(num) => {
                    numbers.push(num);
                }
            }
        }

        if !should_continue {
            break;
        }

        println!("The sum of the entered numbers is: {}", calculate(&numbers, Statistic::Sum));
        println!("The average of the entered numbers is: {}", calculate(&numbers, Statistic::Average));
        println!("The median of the entered numbers is: {}", calculate(&numbers, Statistic::Median));
        println!("The mode of the entered numbers is: {}", calculate(&numbers, Statistic::Mode));

        break;
    }
}
