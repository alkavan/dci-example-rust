pub trait CheckingAccount {
    fn increment(&mut self, amount: f32) -> f32;
}

pub trait SavingsAccount {
    fn decrement(&mut self, amount: f32) -> f32;
}

pub trait AccountLogger {
    fn print_balance(&self);
}
