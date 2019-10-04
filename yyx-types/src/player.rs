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
#[serde(default)]
pub struct PlayerCurrency {
  /// 金币
  pub coin: i64,
  /// 勾玉
  pub jade: i64,
  /// 体力
  pub action_point: i64,
  /// 樱饼
  pub auto_point: i64,
  /// 荣誉
  pub honor: i64,
  /// 勋章
  pub medal: i64,
  /// 功勋
  pub contrib: i64,
  /// 御灵境之钥
  pub totem_pass: i64,
  /// 魂玉
  pub s_jade: i64,
  /// 皮肤券
  pub skin_token: i64,
  /// 突破券
  pub realm_raid_pass: i64,
  /// 破碎的符咒
  pub broken_amulet: i64,
  /// 神秘的符咒
  pub mystery_amulet: i64,
  /// 现世符咒
  pub ar_amulet: i64,
  /// 御札
  pub ofuda: i64,
  /// 金御札
  pub gold_ofuda: i64,
  /// 八岐大蛇鳞片
  pub scale: i64,
  /// 大蛇的逆鳞
  pub reverse_scale: i64,
  /// 逢魔之魂
  pub demon_soul: i64,
  /// 痴念之卷
  pub foolery_pass: i64,
  /// SP皮肤券
  pub sp_skin_token: i64,
}
