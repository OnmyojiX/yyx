use serde_derive::Serialize;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Clone)]
#[serde(tag = "type")]
pub enum AccountId {
  Yyx { server_id: i64, player_id: i64 },
  Cbg { server_id: String, order_sn: String },
}
