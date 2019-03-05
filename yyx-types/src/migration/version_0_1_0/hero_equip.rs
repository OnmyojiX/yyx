use serde_derive::{Deserialize, Serialize};

use super::NextHeroEquip;

use crate::HeroEquipAttr;

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
}

impl From<HeroEquip> for NextHeroEquip {
  fn from(old: HeroEquip) -> NextHeroEquip {
    NextHeroEquip {
      single_attrs: vec![],
      id: old.id, 
      suit_id: old.suit_id, 
      quality: old.quality, 
      pos: old.pos, 
      equip_id: old.equip_id, 
      level: old.level, 
      born: old.born, 
      lock: old.lock, 
      garbage: old.garbage, 
      attrs: old.attrs, 
      base_attr: old.base_attr, 
      random_attrs: old.random_attrs,
      random_attr_rates: old.random_attr_rates,
    }
  }
}
