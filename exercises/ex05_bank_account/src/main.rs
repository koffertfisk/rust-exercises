mod bank_account;

use std::{
    io::{self, Write},
};

use bank_account::BankAccount;

enum NumericInputResult {
    Number(f64),
    Quit,
}

enum TransactionType {
    Withdraw,
    Deposit,
    Balance
}

enum OperationInputResult {
    Transaction(TransactionType),
    Quit
}

fn read_input(prompt: &str) -> NumericInputResult {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            match trimmed.to_lowercase().as_str() {
                "q" | "quit" => return NumericInputResult::Quit,
                _ => match trimmed.parse::<f64>() {
                    Ok(num) => return NumericInputResult::Number(num),
                    Err(_) => println!("Invalid input. Please enter a number or 'q' to quit."),
                },
            }
        } else {
            println!("Failed to read input. Try again.");
        }
    }
}

fn read_operation(prompt: &str) -> OperationInputResult {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            match input.trim().to_lowercase().as_str() {
                "q" | "quit" => return OperationInputResult::Quit,
                "withdraw" => return OperationInputResult::Transaction(TransactionType::Withdraw),
                "deposit" => return OperationInputResult::Transaction(TransactionType::Deposit),
                "balance" => return OperationInputResult::Transaction(TransactionType::Balance),
                _ => println!("Invalid operator. Try again."),
            }
        } else {
            println!("Failed to read input. Try again.");
        }
    }
}

fn handle_transaction(account: &mut BankAccount, tx_type: TransactionType) -> bool {
    match tx_type {
        TransactionType::Balance => {
            println!("Current balance: {}", account.get_balance());
        }
        TransactionType::Deposit => {
            match read_input("Enter amount to deposit: ") {
                NumericInputResult::Quit => return false,
                NumericInputResult::Number(amount) => {
                    account.deposit(amount);
                    println!("{amount} deposited, new balance: {}", account.get_balance());
                }
            }
        },
        TransactionType::Withdraw => {
            match read_input("Enter amount to withdraw: ") {
                NumericInputResult::Quit => return false,
                NumericInputResult::Number(amount) => {
                    if account.withdraw(amount) {
                        println!("{amount} deposited, new balance: {}", account.get_balance());
                    } else {
                        println!(
                            "Insufficient funds; the maximum amount to withdraw is: {}", 
                            account.get_balance()
                        );
                    }
                }
            }
        }
    }

    true
}

fn main() {
    println!("A Simple Bank Account");

    println!("Please enter the name of the account owner:");
    let mut owner = String::new();
    io::stdin().read_line(&mut owner).expect("Failed to read line");
    let owner = owner.trim().to_string();

    let mut account = BankAccount::new(owner);

    loop {
        match read_operation(
            "Enter operation (withdraw, deposit, balance, quit): "
        ) {
            OperationInputResult::Quit => {
                println!("Goodbye!");
                break;
            }
            OperationInputResult::Transaction(tx) => {
                if !handle_transaction(&mut account, tx) {
                    println!("Goodbye!");
                    break;
                }
            }
        }
    }
}
