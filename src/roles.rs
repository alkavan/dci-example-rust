use crate::context::{AccountOperationContext, BankOperationContext};
use crate::domain::Account;

pub trait CheckingAccount {
    fn deposit(account: &mut Account, amount: f64) -> f64;
}

pub trait SavingsAccount {
    fn withdrawal(account: &mut Account, amount: f64) -> f64;
}

pub trait SynchronizedAccount {
    fn lock(account: &mut Account) -> bool;
    fn unlock(account: &mut Account) -> bool;
}

pub trait OperationLogger {
    fn log_account_operation(account_operation: &AccountOperationContext);
    fn log_bank_operation(bank_operation: &BankOperationContext);
}
