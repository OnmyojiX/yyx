use serde_derive::Serialize;
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

  pub fn get_active_states(&self) -> Vec<AccountStateInfo> {
    let lock = self.accounts.read().unwrap();
    lock
      .values()
      .map(|s| {
        let lock = s.read().unwrap();
        AccountStateInfo {
          id: lock.id.clone(),
          busy: false,
        }
      })
      .collect()
  }

  pub fn get_account(&self, id: &AccountId) -> Option<AccountStateRef> {
    self.accounts.read().unwrap().get(id).cloned()
  }

  pub fn open_account(&self, id: AccountId, snapshot: Snapshot) {
    let mut lock = self.accounts.write().unwrap();
    lock.insert(
      id.clone(),
      Arc::new(RwLock::new(AccountState { id, snapshot })),
    );
  }

  pub fn close_account(&self, id: &AccountId, _force: bool) -> bool {
    let mut lock = self.accounts.write().unwrap();
    lock.remove(id);
    true
  }
}

pub struct AccountState {
  pub id: AccountId,
  pub snapshot: Snapshot,
}

#[derive(Debug, Serialize)]
pub struct AccountStateInfo {
  pub id: AccountId,
  pub busy: bool,
}

pub type AccountStateRef = Arc<RwLock<AccountState>>;

pub trait AccountStateExt {
  fn with<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&AccountState) -> R;

  fn get_id(&self) -> AccountId {
    self.with(|state| state.id.clone())
  }
}

impl AccountStateExt for AccountStateRef {
  fn with<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&AccountState) -> R,
  {
    let lock = self.read().unwrap();
    f(&lock)
  }
}
