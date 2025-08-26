use std::fmt;

/*
 * Representing a bank account.
 */
pub struct Account {
    pub(crate) id: u64,
    pub(crate) balance: f64,
    pub(crate) locked: bool,
}

impl Account {
    pub fn new(id: u64, balance: f64, locked: bool) -> Account {
        return Account {
            id,
            balance,
            locked,
        };
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn locked(&self) -> bool {
        self.locked
    }
}

/*
 * Representing a transaction between two bank accounts.
 */
#[derive(Clone, Copy)]
pub struct AccountTransaction {
    amount: f64,
    source_account_id: u64,
    source_account_balance: f64,
    destination_account_id: u64,
    destination_account_balance: f64,
}

impl AccountTransaction {
    pub fn new(
        amount: f64,
        source_account_id: u64,
        source_account_balance: f64,
        destination_account_id: u64,
        destination_account_balance: f64,
    ) -> AccountTransaction {
        AccountTransaction {
            amount,
            source_account_id,
            source_account_balance,
            destination_account_id,
            destination_account_balance,
        }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn source_account_id(&self) -> u64 {
        self.source_account_id
    }

    pub fn source_balance_before(&self) -> f64 {
        self.source_account_balance + self.amount
    }

    pub fn source_balance_after(&self) -> f64 {
        self.source_account_balance
    }

    pub fn destination_account_id(&self) -> u64 {
        self.destination_account_id
    }

    pub fn destination_balance_before(&self) -> f64 {
        self.destination_account_balance - self.amount
    }

    pub fn destination_balance_after(&self) -> f64 {
        self.destination_account_balance
    }
}

/*
 * Representing an error type for account transaction.
 */
pub struct AccountTransactionError {
    pub(crate) account_id: u64,
    pub(crate) message: String,
}

impl AccountTransactionError {
    pub fn new(account_id: u64, message: String) -> AccountTransactionError {
        AccountTransactionError {
            account_id,
            message,
        }
    }
}

#[derive(Clone, Copy)]
pub enum AccountOperation {
    Deposit,
    Withdrawal,
}

#[derive(Clone, Copy)]
pub enum BankOperation {
    MoneyTransfer,
}

/*
 * Trait for operations that can be padded for display.
 */
pub trait OperationFormat {
    fn format(&self) -> String;
}

/*
 * Display trait implementation for account operation.
 */
impl fmt::Display for AccountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountOperation::Deposit => write!(f, "account:deposit"),
            AccountOperation::Withdrawal => write!(f, "account:withdrawal"),
        }
    }
}

impl OperationFormat for AccountOperation {
    fn format(&self) -> String {
        format!("{}", self)
    }
}

/*
 * Display trait implementation for bank operation.
 */
impl fmt::Display for BankOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BankOperation::MoneyTransfer => write!(f, "bank:money_transfer"),
        }
    }
}

impl OperationFormat for BankOperation {
    fn format(&self) -> String {
        format!("{}", self)
    }
}
