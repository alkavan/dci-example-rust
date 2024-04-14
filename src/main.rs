mod data {
    pub struct CheckingAccount {
        identifier: &'static str,
        balance: f32,
    }

    impl CheckingAccount {
        pub fn new(identifier: &'static str, balance: f32) -> CheckingAccount {
            return CheckingAccount {
                identifier,
                balance,
            };
        }

        pub fn identifier(&self) -> &str {
            return self.identifier;
        }

        pub fn balance(&self) -> f32 {
            return self.balance.clone();
        }

        pub fn increment(&mut self, amount: f32) -> f32 {
            self.balance += amount;
            return self.balance.clone();
        }
    }
}

fn main() {
    let mut account = data::CheckingAccount::new("account#420", 1_000_000.0);

    println!("account balance: {:.6}", account.balance());
    account.increment(500_000.0);
    println!("account balance: {:.6}", account.balance());
}
