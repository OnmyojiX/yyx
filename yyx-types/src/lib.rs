//! 痒痒熊生成的快照的类型定义
//!

#![warn(clippy::all)]

use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};

mod hero;
mod hero_book_shard;
mod hero_equip;
mod hero_equip_preset;
mod player;

pub use self::hero::*;
pub use self::hero_book_shard::*;
pub use self::hero_equip::*;
pub use self::hero_equip_preset::*;
pub use self::player::*;

/// 痒痒熊生成的快照
#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
  pub version: String,
  pub timestamp: DateTime<Local>,
  pub data: SnapshotData,
}

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
