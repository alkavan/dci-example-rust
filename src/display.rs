use crate::domain::{AccountOperation, BankOperation};
use std::fmt;

impl fmt::Display for AccountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountOperation::Deposit => write!(f, "{:<20}", "account:deposit"),
            AccountOperation::Withdrawal => write!(f, "{:<20}", "account:withdrawal"),
        }
    }
}

impl fmt::Display for BankOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BankOperation::MoneyTransfer => write!(f, "{:<20}", "bank:money_transfer"),
        }
    }
}
