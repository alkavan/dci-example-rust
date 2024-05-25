mod context;
mod display;
mod domain;
mod roles;

#[macro_use]
extern crate log;

use domain::Account;

use context::BankContext;
use context::{AccountMap, MoneyTransferContext, MoneyTransferQueue};

fn main() {
    pretty_env_logger::init();

    // account ids
    let account1_id = 7777u64;
    let account2_id = 8888u64;
    let account3_id = 9999u64;

    // this map will store account in the required lifetime.
    let mut accounts = AccountMap::new();

    // account objects are being moved here and cannot be used in this scope anymore.
    accounts.insert(7777, Account::new(7777, 1_000_000.0, false));
    accounts.insert(8888, Account::new(8888, 1_000_000.0, false));
    accounts.insert(9999, Account::new(9999, 1_000_000.0, false));

    // money transfer queue
    let mut transfer_queue = MoneyTransferQueue::new();

    // create bank entity
    let mut bank = BankContext::new(&mut accounts, &mut transfer_queue);

    // money transfer 1
    let transfer = MoneyTransferContext::new(42_000.0, account1_id, account3_id);
    bank.account_transfer(transfer);

    // money transfer 2
    let transfer = MoneyTransferContext::new(69_000.0, account2_id, account3_id);
    bank.account_transfer(transfer);

    // money transfer 3
    let transfer = MoneyTransferContext::new(96_000.0, account3_id, account1_id);
    bank.account_transfer(transfer);

    // in some later time, execute all transfers in bank queue.
    bank.apply_a2a_transfers();
}
