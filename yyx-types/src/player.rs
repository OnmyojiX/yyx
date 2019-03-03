use serde_derive::{Deserialize, Serialize};

/// 玩家信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
  /// ID
  pub id: i64,
  /// 名称
  pub name: String,
  /// 等级
  pub level: i64,
}

/// 玩家资源
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerCurrency {
  /// 金币
  pub coin: i64,
  /// 勾玉
  pub jade: i64,
  /// 体力
  pub action_point: i64,
}
