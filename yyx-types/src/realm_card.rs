use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmCard {
  pub id: String,
  pub item_id: i64,
  pub total_time: i64,
  pub attrs: RealmCardAttr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmCardAttr {
  pub exp: i64,
  pub bonus: i64,
}