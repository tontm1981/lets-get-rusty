pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    pub fn new () -> Self {
        SavingsAccount {
            balance: 0,
        }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    pub fn transfer(&self, acc_number: i32, amount:i32) -> Result<String, String> {
        Ok(format!("Transferred {amount} to account {acc_number}"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_0() {
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100);
    }

    #[test]
    fn should_transfer_money() {
        let account = SavingsAccount::new();
        let result = account.transfer(1234, 100);
        assert_eq!(result, Ok("Transferred 100 to account 1234".to_string()));
    }

    #[test]
    fn another_way_should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        account.transfer(1234, 100)?;
        Ok(())
    }
}