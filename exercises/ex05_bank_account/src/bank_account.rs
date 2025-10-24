#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
    pub owner: String
}

impl BankAccount {
    pub fn new(owner: String) -> Self {
        BankAccount { balance: 0.0, owner }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance = self.balance + amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance - amount >= 0.0 {
            self.balance = self.balance - amount;
            return true
        }

        return false
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_and_withdraw() {
        let mut account = BankAccount::new("Test".to_string());

        account.deposit(100.0);
        assert_eq!(account.get_balance(), 100.0);

        assert!(account.withdraw(50.0));
        assert_eq!(account.get_balance(), 50.0);

        assert!(!account.withdraw(100.0));
        assert_eq!(account.get_balance(), 50.0);
    }
}