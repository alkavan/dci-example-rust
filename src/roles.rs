use crate::context::{
    AccountOperationContext, BankContext, BankOperationContext, MoneyTransferContext,
};
use crate::domain::BankOperation::MoneyTransfer;
use crate::domain::{Account, AccountOperation, AccountTransaction, BankOperation};

// Account role for making a money deposit
pub trait CheckingAccount {
    fn deposit(account: &mut Account, amount: f64) -> f64;
}

impl CheckingAccount for Account {
    fn deposit(account: &mut Account, amount: f64) -> f64 {
        account.balance += amount;
        account.balance.clone()
    }
}

// Account role for making a money withdrawal
impl SavingsAccount for Account {
    fn withdrawal(account: &mut Account, amount: f64) -> f64 {
        account.balance -= amount;
        account.balance.clone()
    }
}

pub trait SavingsAccount {
    fn withdrawal(account: &mut Account, amount: f64) -> f64;
}

// Account role for locking the account for operations
impl SynchronizedAccount for Account {
    fn lock(account: &mut Account) -> bool {
        // do operation only if account is unlocked.
        if !account.locked() {
            account.locked = true;
            return account.locked;
        }
        return false;
    }

    fn unlock(account: &mut Account) -> bool {
        // do operation only if account is locked.
        if account.locked() {
            account.locked = false;
            return true;
        }
        return false;
    }
}

pub trait SynchronizedAccount {
    fn lock(account: &mut Account) -> bool;
    fn unlock(account: &mut Account) -> bool;
}

// Account operation logger
pub trait AccountOperationLogger {
    fn log(operation: &AccountOperation, transaction: &AccountTransaction);
}

impl AccountOperationLogger for BankContext<'_> {
    fn log(operation: &AccountOperation, transaction: &AccountTransaction) {
        match operation {
            AccountOperation::Deposit => {
                info!(
                    "[{}] account#{} {:.6} + {:.6} = {:.6}",
                    operation,
                    transaction.source_account_id(),
                    transaction.source_balance_before(),
                    transaction.amount(),
                    transaction.source_balance_after()
                );
            }
            AccountOperation::Withdrawal => {
                info!(
                    "[{}] account#{} {:.6} - {:.6} = {:.6}",
                    operation,
                    transaction.destination_account_id(),
                    transaction.destination_balance_before(),
                    transaction.amount(),
                    transaction.destination_balance_after()
                );
            }
        }
    }
}

// Bank operation logger
pub trait BankOperationLogger {
    fn log(operation: &BankOperation, transaction: &AccountTransaction);
}

impl BankOperationLogger for BankContext<'_> {
    fn log(operation: &BankOperation, transaction: &AccountTransaction) {
        match operation {
            MoneyTransfer => {
                info!(
                    "[{}] transferred {:.6} from account#{} to account#{}",
                    operation,
                    transaction.amount(),
                    transaction.source_account_id(),
                    transaction.destination_account_id()
                );
            }
        }
    }
}
