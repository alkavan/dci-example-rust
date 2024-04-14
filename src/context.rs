extern crate pretty_env_logger;

use crate::roles::{AccountLogger, CheckingAccount, SavingsAccount};
use crate::Account;

use std::collections::HashMap;
use std::collections::VecDeque;

impl AccountLogger for Account {
    fn print_balance(&self) {
        info!("balance of {}: {:.6}", self.id(), self.balance());
    }
}

impl CheckingAccount for Account {
    fn increment(&mut self, amount: f64) -> f64 {
        self.balance += amount;
        return self.balance.clone();
    }
}

impl SavingsAccount for Account {
    fn decrement(&mut self, amount: f64) -> f64 {
        self.balance -= amount;
        return self.balance.clone();
    }
}

pub struct MoneyTransferContext<'a> {
    source: &'a mut Account,
    destination: &'a mut Account,
    amount: f64,
}

impl MoneyTransferContext<'_> {
    pub fn new<'a>(
        amount: f64,
        source: &'a mut Account,
        destination: &'a mut Account,
    ) -> MoneyTransferContext<'a> {
        MoneyTransferContext {
            source,
            destination,
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
            self.source.id(),
            self.destination.id()
        );

        self.source.print_balance();
        self.destination.print_balance();
    }
}

/*
 * represents a list of account, not actually part of the business domain.
 */
pub(crate) type AccountMap = HashMap<u64, Account>;

/*
 * a bank use-case using the money transfer context.
 */
pub struct BankContext<'a> {
    accounts: &'a mut AccountMap,
    transfer_queue: VecDeque<MoneyTransferContext<'a>>,
}

impl BankContext<'_> {
    pub fn new(accounts: &mut AccountMap) -> BankContext {
        let transfer_queue = VecDeque::new();

        BankContext {
            accounts,
            transfer_queue,
        }
    }

    pub fn account_transfer<'a>(&mut self, amount: f64, source: u64, destination: u64) {
        let transfer = MoneyTransferContext::new(
            amount,
            self.accounts.get(*source),
            self.accounts.get(*destination),
        );
        self.transfer_queue.append(transfer);
    }

    pub fn apply_a2a_transfers<'a>(&mut self) {}
}
