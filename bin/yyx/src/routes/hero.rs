use crate::helpers::*;
use crate::result::*;

use yyx_types::Hero;

#[get("/hero")]
pub fn list(snapshot: YyxResult<SnapshotRef>) -> YyxResult<JsonString<Vec<Hero>>> {
  let s = snapshot?;
  JsonString::new(&s.data.heroes)
}
