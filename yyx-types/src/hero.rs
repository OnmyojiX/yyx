use serde_derive::{Deserialize, Serialize};

/// 式神
#[derive(Debug, Serialize, Deserialize)]
pub struct Hero {
  /// ID
  pub id: String,
  /// 类型ID
  pub hero_id: i64,
  /// 装备的御魂列表
  pub equips: Vec<String>,
  /// 等级
  pub level: i64,
  /// 经验值
  pub exp: f64,
  /// 经验比率
  pub exp_rate: f64,
  /// 昵称
  pub nick_name: String,
  /// 创建时间戳
  pub born: i64,
  /// 是否锁定
  pub lock: bool,
  /// 稀有度
  pub rarity: HeroRarity,
  /// 技能列表
  pub skills: Vec<HeroSkill>,
}

/// 技能
#[derive(Debug, Serialize, Deserialize)]
pub struct HeroSkill {
  /// 类型ID
  pub id: i64,
  /// 级别
  pub level: i64,
}

/// 式神稀有度
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum HeroRarity {
  N,
  R,
  SR,
  SSR,
  SP,
}
