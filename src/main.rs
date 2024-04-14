mod context;
mod data;
mod roles;

#[macro_use]
extern crate log;

use context::MoneyTransferContext;
use data::Account;

fn main() {
    pretty_env_logger::init();

    let mut account1 = Account::new("account#420", 1_000_000.0);
    let mut account2 = Account::new("account#720", 1_000_000.0);

    let mut transfer = MoneyTransferContext::new(&mut account1, &mut account2, 150_000.0);
    transfer.transfer();
}
