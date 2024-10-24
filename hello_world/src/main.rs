#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
    }

    // Add more tests here
}
