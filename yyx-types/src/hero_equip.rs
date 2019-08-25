use serde_derive::{Deserialize, Serialize};

/// 御魂属性类型
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum HeroEquipAttrType {
  /// 生命
  Hp,
  /// 防御
  Defense,
  /// 攻击
  Attack,
  /// 生命加成
  HpRate,
  /// 防御加成
  DefenseRate,
  /// 攻击加成
  AttackRate,
  /// 速度
  Speed,
  /// 暴击
  CritRate,
  /// 暴击伤害
  CritPower,
  /// 效果命中
  EffectHitRate,
  /// 效果抵抗
  EffectResistRate,
}

/// 御魂
#[derive(Debug, Serialize, Deserialize)]
pub struct HeroEquip {
  /// ID
  pub id: String,
  /// 套装类型ID
  pub suit_id: i64,
  /// 星级(1-6)
  pub quality: i64,
  /// 位置(0-5)
  pub pos: i64,
  pub equip_id: i64,
  /// 强化等级
  pub level: i64,
  /// 获取时间戳
  pub born: i64,
  /// 是否锁定
  pub lock: bool,
  /// 是否弃置
  pub garbage: bool,
  /// 属性列表
  pub attrs: Vec<HeroEquipAttr>,
  /// 基础属性
  pub base_attr: HeroEquipAttr,
  /// 随机属性列表
  pub random_attrs: Vec<HeroEquipAttr>,
  /// 随机属性强化比率
  pub random_attr_rates: Vec<HeroEquipAttr>,
  /// 单个御魂的属性（首领御魂）
  #[serde(default)]
  pub single_attrs: Vec<HeroEquipAttr>,
}

/// 御魂属性
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeroEquipAttr {
  /// 属性类型
  #[serde(rename = "type")]
  pub type_: HeroEquipAttrType,
  /// 值
  pub value: f64,
}
