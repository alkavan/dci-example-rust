use crate::context::BankContext;
use crate::domain::{
    Account, AccountOperation, AccountTransaction, BankOperation, OperationFormat,
};

/*
 * Use-case for an account making a money deposit
 */
pub trait CheckingAccount {
    fn deposit(account: &mut Account, amount: f64) -> f64;
}

impl CheckingAccount for Account {
    fn deposit(account: &mut Account, amount: f64) -> f64 {
        account.balance += amount;
        account.balance()
    }
}

/*
 * Use-case for an account making a money withdrawal
 */
impl SavingsAccount for Account {
    fn withdrawal(account: &mut Account, amount: f64) -> f64 {
        account.balance -= amount;
        account.balance()
    }
}

pub trait SavingsAccount {
    fn withdrawal(account: &mut Account, amount: f64) -> f64;
}

/*
 * Use-case for an account that has locking/unlocking capabilities
 */
pub trait SynchronizedAccount {
    fn lock(account: &mut Account) -> bool;
    fn unlock(account: &mut Account) -> bool;
}

impl SynchronizedAccount for Account {
    fn lock(account: &mut Account) -> bool {
        // do operation only if an account is unlocked.
        if !account.locked() {
            account.locked = true;
            return true;
        }
        false
    }

    fn unlock(account: &mut Account) -> bool {
        if account.locked() {
            account.locked = false;
            return true;
        }
        false
    }
}

/*
 * Use-case for bank logging account operations;
 * e.g., money deposit, money withdrawal.
 */
pub trait AccountOperationLogger {
    fn log(operation: &AccountOperation, transaction: &AccountTransaction);
}

impl AccountOperationLogger for BankContext<'_> {
    fn log(operation: &AccountOperation, transaction: &AccountTransaction) {
        match operation {
            AccountOperation::Deposit => {
                info!(
                    "[{:>20}] account #{} {:.6} + {:.6} = {:.6}",
                    operation.format(),
                    transaction.source_account_id(),
                    transaction.source_balance_before(),
                    transaction.amount(),
                    transaction.source_balance_after()
                );
            }
            AccountOperation::Withdrawal => {
                info!(
                    "[{:>20}] account #{} {:.6} - {:.6} = {:.6}",
                    operation.format(),
                    transaction.destination_account_id(),
                    transaction.destination_balance_before(),
                    transaction.amount(),
                    transaction.destination_balance_after()
                );
            }
        }
    }
}

/*
 * Use-case for a bank logging bank operations;
 * e.g., money transfer, money withdrawal, money deposit.
 */
pub trait BankOperationLogger {
    fn log(operation: &BankOperation, transaction: &AccountTransaction);
}

impl BankOperationLogger for BankContext<'_> {
    fn log(operation: &BankOperation, transaction: &AccountTransaction) {
        match operation {
            BankOperation::MoneyTransfer => {
                info!(
                    "[{:>20}] transferred {:.6} from account #{} to account #{}",
                    operation.format(),
                    transaction.amount(),
                    transaction.source_account_id(),
                    transaction.destination_account_id()
                );
            }
        }
    }
}
