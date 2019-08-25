use serde_derive::{Deserialize, Serialize};

/// 玩家信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
  /// ID
  pub id: i64,
  /// 服务器
  #[serde(default)]
  pub server_id: i64,
  /// 名称
  pub name: String,
  /// 等级
  pub level: i64,
}

/// 玩家资源
///
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PlayerCurrency {
  /// 金币
  pub coin: i64,
  /// 勾玉
  pub jade: i64,
  /// 体力
  pub action_point: i64,
  /// 樱饼
  #[serde(default)]
  pub auto_point: i64,
  /// 荣誉
  #[serde(default)]
  pub honor: i64,
  /// 勋章
  #[serde(default)]
  pub medal: i64,
  /// 功勋
  #[serde(default)]
  pub contrib: i64,
  /// 御灵境之钥
  #[serde(default)]
  pub totem_pass: i64,
  /// 魂玉
  #[serde(default)]
  pub s_jade: i64,
  /// 皮肤券
  #[serde(default)]
  pub skin_token: i64,
  /// 突破券
  #[serde(default)]
  pub realm_raid_pass: i64,
}
