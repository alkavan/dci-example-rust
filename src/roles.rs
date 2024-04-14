pub trait CheckingAccount {
    fn increment(&mut self, amount: f64) -> f64;
}

pub trait SavingsAccount {
    fn decrement(&mut self, amount: f64) -> f64;
}

pub trait AccountLogger {
    fn print_balance(&self);
}
