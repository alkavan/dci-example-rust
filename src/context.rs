extern crate pretty_env_logger;

use crate::domain::{
    Account, AccountOperation, AccountTransaction, AccountTransactionError, BankOperation,
};
use crate::roles::{
    AccountOperationLogger, BankOperationLogger, CheckingAccount, SavingsAccount,
    SynchronizedAccount,
};

use crate::domain::BankOperation::MoneyTransfer;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct AccountOperationContext {
    pub(crate) operation: AccountOperation,
    pub(crate) account_id: u64,
    pub(crate) amount: Option<f64>,
    pub(crate) balance: Option<f64>,
}

impl AccountOperationContext {
    pub fn new(
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

pub struct BankOperationContext {
    pub(crate) operation: BankOperation,
    pub(crate) source_account_id: u64,
    pub(crate) destination_account_id: u64,
    pub(crate) amount: Option<f64>,
}

impl BankOperationContext {
    pub fn new(
        amount: f64,
        source_account_id: u64,
        destination_account_id: u64,
    ) -> BankOperationContext {
        BankOperationContext {
            operation: MoneyTransfer,
            source_account_id,
            destination_account_id,
            amount: Some(amount),
        }
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

    pub fn transfer(
        &mut self,
        accounts: &mut AccountMap,
    ) -> Result<AccountTransaction, AccountTransactionError> {
        let amount = self.amount.clone();

        let source = accounts.get_mut(&self.source_account_id).unwrap();

        // withdrawal from source account
        if !Account::lock(source) {
            return Err(AccountTransactionError::new(
                source.id(),
                format!("account#{} is locked", self.source_account_id),
            ));
        }
        let source_account_id = source.id();
        let source_balance_after = Account::withdrawal(source, amount);
        Account::unlock(source);

        let destination = accounts.get_mut(&self.destination_account_id).unwrap();

        // deposit to destination account
        if !Account::lock(destination) {
            return Err(AccountTransactionError::new(
                destination.id(),
                format!("account#{} is locked", self.destination_account_id),
            ));
        }
        let destination_account_id = destination.id();
        let destination_balance_after = Account::deposit(destination, amount);
        Account::unlock(destination);

        Ok(AccountTransaction::new(
            amount,
            source_account_id,
            source_balance_after,
            destination_account_id,
            destination_balance_after,
        ))
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
            let maybe_transaction = money_transfer_context.transfer(self.accounts);

            if maybe_transaction.is_err() {
                let err = maybe_transaction.err().unwrap();
                info!("[error] account#{} : {}", err.account_id, err.message);
                continue;
            }

            let transaction = maybe_transaction.ok().unwrap();

            <BankContext<'_> as AccountOperationLogger>::log(
                &AccountOperation::Withdrawal,
                &transaction,
            );
            <BankContext<'_> as AccountOperationLogger>::log(
                &AccountOperation::Deposit,
                &transaction,
            );
            <BankContext<'_> as BankOperationLogger>::log(&MoneyTransfer, &transaction);
        }
    }
}
