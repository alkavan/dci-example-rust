use std::collections::HashMap;

pub struct Account {
    pub(crate) id: u64,
    pub(crate) balance: f64,
    pub(crate) locked: bool,
}

impl Account {
    pub fn new(id: u64, balance: f64) -> Account {
        return Account {
            id,
            balance,
            locked: false,
        };
    }

    pub fn id(&self) -> u64 {
        return self.id;
    }

    pub fn balance(&self) -> f64 {
        return self.balance.clone();
    }
}
