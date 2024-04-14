mod context;
mod domain;
mod roles;

#[macro_use]
extern crate log;

use domain::Account;

use crate::context::AccountMap;
use context::BankContext;

fn main() {
    pretty_env_logger::init();

    let mut accounts = AccountMap::new();

    // create accounts with $1m in balance.
    let account1 = Account::new(7777, 1_000_000.0);
    let account2 = Account::new(8888, 1_000_000.0);
    let account3 = Account::new(9999, 1_000_000.0);

    // account objects are being moved here and cannot be used in this scope anymore.
    accounts.insert(account1.id(), account1);
    accounts.insert(account2.id(), account2);
    accounts.insert(account3.id(), account3);

    let mut bank = BankContext::new(&mut accounts);

    bank.account_transfer(100_000.0, 7777, 9999);
}
