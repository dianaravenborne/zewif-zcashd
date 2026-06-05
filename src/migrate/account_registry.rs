use std::collections::HashMap;

use zewif::Account;

use crate::zcashd_wallet::UfvkFingerprint;

#[allow(dead_code)]
struct AccountRegistry {
    accounts: Vec<Account>,
    key_index: HashMap<UfvkFingerprint, usize>,
}

impl AccountRegistry {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        AccountRegistry {
            accounts: vec![],
            key_index: HashMap::new(),
        }
    }
}
