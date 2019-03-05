use serde_derive::{Deserialize, Serialize};

const NEXT_VERSION: &str = "0.1.1";
pub type NextSnapshotData = crate::SnapshotData;
pub type NextHeroEquip = crate::HeroEquip;

mod hero_equip;

use crate::{Hero, HeroBookShard, HeroEquipPreset, Player, PlayerCurrency};

use self::hero_equip::HeroEquip;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotData {
  /// 玩家信息
  pub player: Player,
  /// 玩家货币
  pub currency: PlayerCurrency,
  /// 所有的式神
  pub heroes: Vec<Hero>,
  /// 所有的御魂
  pub hero_equips: Vec<HeroEquip>,
  /// 所有的御魂方案
  pub hero_equip_presets: Vec<HeroEquipPreset>,
  /// 所有的式神召唤书碎片
  pub hero_book_shards: Vec<HeroBookShard>,
}

impl From<SnapshotData> for NextSnapshotData {
  fn from(old: SnapshotData) -> Self {
    NextSnapshotData {
      player: old.player,
      currency: old.currency,
      heroes: old.heroes,
      hero_equips: old.hero_equips.into_iter().map(Into::into).collect(),
      hero_equip_presets: old.hero_equip_presets,
      hero_book_shards: old.hero_book_shards,
      realm_cards: vec![],
    }
  }
}

impl crate::migration::Migration for SnapshotData {
  const FROM_VERSION: &'static str = "0.1.0";
  const TO_VERSION: &'static str = NEXT_VERSION;

  type To = NextSnapshotData;
}
