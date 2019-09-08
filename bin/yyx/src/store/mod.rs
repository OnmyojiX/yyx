use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use yyx_data::account_id::AccountId;
use yyx_types::Snapshot;

pub struct YyxStore {
  accounts: RwLock<HashMap<AccountId, AccountStateRef>>,
}

pub type YyxStoreRef = Arc<YyxStore>;

impl YyxStore {
  pub fn new_ref() -> YyxStoreRef {
    Arc::new(YyxStore {
      accounts: RwLock::new(HashMap::new()),
    })
  }

  pub fn get_account(&self, id: &AccountId) -> Option<AccountStateRef> {
    self.accounts.read().unwrap().get(id).cloned()
  }

  pub fn set_account(&self, id: AccountId, snapshot: Snapshot) {
    let mut lock = self.accounts.write().unwrap();
    lock.insert(id, Arc::new(RwLock::new(AccountState { snapshot })));
  }
}

pub struct AccountState {
  pub snapshot: Snapshot,
}

pub type AccountStateRef = Arc<RwLock<AccountState>>;
