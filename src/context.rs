extern crate pretty_env_logger;

use crate::domain::{Account, AccountOperation};
use crate::roles::{CheckingAccount, OperationLogger, SavingsAccount, SynchronizedAccount};

use std::collections::HashMap;
use std::collections::VecDeque;

pub struct AccountOperationContext {
    operation: AccountOperation,
    account_id: u64,
    amount: Option<f64>,
    balance: Option<f64>,
}

impl AccountOperationContext {
    pub fn new_transfer(
        operation: AccountOperation,
        account_id: u64,
        amount: f64,
        balance: f64,
    ) -> AccountOperationContext {
        AccountOperationContext {
            operation,
            account_id,
            amount: Some(amount),
            balance: Some(balance),
        }
    }
}

impl CheckingAccount for Account {
    fn deposit(account: &mut Account, amount: f64) -> f64 {
        account.balance += amount;
        account.balance.clone()
    }
}

impl SavingsAccount for Account {
    fn withdrawal(account: &mut Account, amount: f64) -> f64 {
        account.balance -= amount;
        account.balance.clone()
    }
}

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

pub struct MoneyTransferContext {
    source_account_id: u64,
    destination_account_id: u64,
    amount: f64,
}

impl MoneyTransferContext {
    pub fn new(
        amount: f64,
        source_account_id: u64,
        destination_account_id: u64,
    ) -> MoneyTransferContext {
        MoneyTransferContext {
            source_account_id,
            destination_account_id,
            amount,
        }
    }

    pub fn transfer(&mut self, accounts: &mut AccountMap) {
        let source = accounts.get_mut(&self.source_account_id).unwrap();
        let amount = self.amount.clone();

        Account::lock(source);
        Account::withdrawal(source, amount);
        Account::unlock(source);

        MoneyTransferContext::log_account_operation(AccountOperationContext::new_transfer(
            AccountOperation::Withdrawal,
            source.id(),
            amount,
            source.balance(),
        ));

        let destination = accounts.get_mut(&self.destination_account_id).unwrap();
        Account::lock(destination);
        Account::deposit(destination, amount);
        Account::lock(destination);

        MoneyTransferContext::log_account_operation(AccountOperationContext::new_transfer(
            AccountOperation::Deposit,
            destination.id(),
            amount,
            destination.balance(),
        ));

        info!(
            "transferred {:.6} from account#{} to account#{}",
            self.amount, self.source_account_id, self.destination_account_id
        );
    }
}

impl OperationLogger for MoneyTransferContext {
    fn log_account_operation(account_operation: AccountOperationContext) {
        let operation = account_operation.operation.clone();
        let account_id = account_operation.account_id;

        match operation {
            AccountOperation::Deposit => {
                let account_balance = account_operation.balance.unwrap();
                let amount = account_operation.amount.unwrap();
                let account_balance_before = account_balance - amount;
                info!(
                    "account#{} [{}] {:.6} + {:.6} = {:.6}",
                    account_id, operation, account_balance_before, amount, account_balance
                );
            }
            AccountOperation::Withdrawal => {
                let account_balance = account_operation.balance.unwrap();
                let amount = account_operation.amount.unwrap();
                let account_balance_before = account_balance + amount;
                info!(
                    "account#{} [{}] {:.6} - {:.6} = {:.6}",
                    account_id, operation, account_balance_before, amount, account_balance
                );
            }
        }
    }
}

/*
 * represents a list of account, not actually part of the business domain.
 */
pub(crate) type AccountMap = HashMap<u64, Account>;
pub(crate) type MoneyTransferQueue<'a> = VecDeque<MoneyTransferContext>;

/*
 * a bank use-case using the money transfer context.
 */
pub struct BankContext<'a> {
    accounts: &'a mut AccountMap,
    transfer_queue: &'a mut MoneyTransferQueue<'a>,
}

impl BankContext<'_> {
    pub fn new<'a>(
        accounts: &'a mut HashMap<u64, Account>,
        transfer_queue: &'a mut MoneyTransferQueue<'a>,
    ) -> BankContext<'a> {
        BankContext {
            accounts,
            transfer_queue,
        }
    }

    pub fn account_transfer(&mut self, transfer: MoneyTransferContext) {
        self.transfer_queue.push_back(transfer);
    }

    pub fn apply_a2a_transfers(&mut self) {
        while let Some(mut money_transfer_context) = self.transfer_queue.pop_front() {
            money_transfer_context.transfer(self.accounts);
        }
    }
}
