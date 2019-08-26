use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CbgRealmCard {
  pub num: i64,
  pub star: i64,
}

#[derive(Debug, Deserialize)]
pub struct CbgHeroBookShard {
  pub hero_id: i64,
  pub num: i64,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum CbgAttrType {
  #[serde(rename(deserialize = "maxHpAdditionVal", deserialize = "生命"))]
  Hp,
  #[serde(rename(deserialize = "defenseAdditionVal", deserialize = "防御"))]
  Defense,
  #[serde(rename(deserialize = "attackAdditionVal", deserialize = "攻击"))]
  Attack,
  #[serde(rename(deserialize = "maxHpAdditionRate", deserialize = "生命加成"))]
  HpRate,
  #[serde(rename(deserialize = "defenseAdditionRate", deserialize = "防御加成"))]
  DefenseRate,
  #[serde(rename(deserialize = "attackAdditionRate", deserialize = "攻击加成"))]
  AttackRate,
  #[serde(rename(deserialize = "speedAdditionVal", deserialize = "速度"))]
  Speed,
  #[serde(rename(deserialize = "critRateAdditionVal", deserialize = "暴击"))]
  CritRate,
  #[serde(rename(deserialize = "critPowerAdditionVal", deserialize = "暴击伤害"))]
  CritPower,
  #[serde(rename(deserialize = "debuffEnhance", deserialize = "效果命中"))]
  EffectHitRate,
  #[serde(rename(deserialize = "debuffResist", deserialize = "效果抵抗"))]
  EffectResistRate,
}

#[derive(Debug, Deserialize)]
pub struct CbgStringAttrValue(pub(crate) String);

#[derive(Debug, Deserialize)]
pub struct CbgEquip {
  #[serde(rename = "uuid")]
  pub id: String,
  #[serde(rename = "herouid")]
  pub equipped_by: Option<String>,
  #[serde(rename = "suitid")]
  pub suit_id: i64,
  #[serde(rename = "itemId")]
  pub equip_id: i64,
  pub level: i64,
  pub pos: i64,
  #[serde(rename = "qua")]
  pub quality: i64,
  #[serde(default)]
  pub lock: bool,
  #[serde(default)]
  #[serde(rename = "isuseless")]
  pub garbage: bool,
  pub rattr: Vec<(CbgAttrType, f64)>,
  pub attrs: Vec<(CbgAttrType, CbgStringAttrValue)>,
  pub single_attr: Option<(CbgAttrType, CbgStringAttrValue)>,
}

#[derive(Debug, Deserialize)]
pub struct CbgHeroRarity(pub(crate) u8);

#[derive(Debug, Deserialize)]
pub struct CbgHeroAttr {
  pub val: CbgStringAttrValue,
  pub add_val: Option<CbgStringAttrValue>,
}

#[derive(Debug, Deserialize)]
pub struct CbgHero {
  #[serde(rename = "uid")]
  pub id: String,
  #[serde(rename = "heroId")]
  pub hero_id: i64,
  pub star: i64,
  pub level: i64,
  #[serde(default)]
  pub lock: bool,
  pub nick: Option<String>,
  pub rarity: CbgHeroRarity,
  #[serde(default)]
  pub born: i64,
  pub awake: i64,
  pub equips: Vec<String>,
  #[serde(rename = "selectSkills")]
  pub skills: Vec<i64>,
  #[serde(default)]
  pub exp: f64,
  pub attrs: HashMap<CbgAttrType, CbgHeroAttr>,
}

#[derive(Debug, Deserialize)]
pub struct CbgEquipDesc {
  #[serde(rename = "on_sell_sn")]
  pub id: String,

  #[serde(rename = "name")]
  pub player_name: String,
  #[serde(rename = "lv")]
  pub player_level: i64,

  #[serde(rename = "money")]
  pub coin: i64,
  #[serde(rename = "goyu")]
  pub jade: i64,
  #[serde(rename = "strength")]
  pub action_point: i64,
  #[serde(rename = "currency_900273")]
  pub auto_point: i64,
  #[serde(rename = "honor_score")]
  pub honor: i64,
  pub medal: i64,
  #[serde(rename = "currency_900215")]
  pub totem_pass: i64,
  #[serde(rename = "hunyu")]
  pub s_jade: i64,
  #[serde(rename = "skin_coupon")]
  pub skin_token: i64,

  #[serde(rename = "lbscards")]
  pub realm_cards: HashMap<String, CbgRealmCard>,

  #[serde(rename = "hero_fragment")]
  pub hero_book_shards: HashMap<String, CbgHeroBookShard>,

  #[serde(rename = "inventory")]
  pub equips: HashMap<String, CbgEquip>,

  pub heroes: HashMap<String, CbgHero>,
}
