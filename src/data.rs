pub struct Account {
    identifier: &'static str,
    pub(crate) balance: f32,
}

impl Account {
    pub fn new(identifier: &'static str, balance: f32) -> Account {
        return Account {
            identifier,
            balance,
        };
    }

    pub fn identifier(&self) -> &str {
        return self.identifier;
    }

    pub fn balance(&self) -> f32 {
        return self.balance.clone();
    }
}
