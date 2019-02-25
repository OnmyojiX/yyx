use serde_derive::{Deserialize, Serialize};

/// 御魂方案
#[derive(Debug, Serialize, Deserialize)]
pub struct HeroEquipPreset {
  /// 方案名
  pub name: String,
  /// 方案御魂ID列表
  pub items: Vec<String>,
}
