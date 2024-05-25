use std::fmt;

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

#[derive(Clone, Copy)]
pub enum AccountOperation {
    Deposit,
    Withdrawal,
}

impl fmt::Display for AccountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountOperation::Deposit => write!(f, "deposit"),
            AccountOperation::Withdrawal => write!(f, "withdrawal"),
        }
    }
}
