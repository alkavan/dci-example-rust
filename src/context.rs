extern crate pretty_env_logger;

use crate::roles::{AccountLogger, CheckingAccount, SavingsAccount};
use crate::Account;

impl AccountLogger for Account {
    fn print_balance(&self) {
        info!("balance of {}: {:.6}", self.identifier(), self.balance());
    }
}

impl CheckingAccount for Account {
    fn increment(&mut self, amount: f32) -> f32 {
        self.balance += amount;
        return self.balance.clone();
    }
}

impl SavingsAccount for Account {
    fn decrement(&mut self, amount: f32) -> f32 {
        self.balance -= amount;
        return self.balance.clone();
    }
}

pub struct MoneyTransferContext<'a> {
    source: &'a mut Account,
    destination: &'a mut Account,
    amount: f32,
}

impl MoneyTransferContext<'_> {
    pub fn new<'a>(
        from: &'a mut Account,
        to: &'a mut Account,
        amount: f32,
    ) -> MoneyTransferContext<'a> {
        MoneyTransferContext {
            source: from,
            destination: to,
            amount,
        }
    }

    pub fn transfer(&mut self) {
        self.source.print_balance();
        self.destination.print_balance();

        self.source.decrement(self.amount.clone());
        self.destination.increment(self.amount.clone());

        info!(
            "transferred {:.6} from {} to {}",
            self.amount,
            self.source.identifier(),
            self.destination.identifier()
        );

        self.source.print_balance();
        self.destination.print_balance();
    }
}
