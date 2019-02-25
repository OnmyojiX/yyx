use crate::helpers::*;
use crate::result::*;

use yyx_types::HeroEquip;

#[get("/equip")]
pub fn list<'r>(snapshot: YyxResult<SnapshotRef>) -> YyxResult<JsonString<Vec<HeroEquip>>> {
  let s = snapshot?;
  JsonString::new(&s.data.hero_equips)
}
