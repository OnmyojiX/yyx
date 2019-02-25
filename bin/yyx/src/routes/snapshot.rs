use crate::helpers::*;
use crate::result::*;

use yyx_data::save_last_snapshot;
use yyx_types::Snapshot;

#[put("/snapshot", data = "<body>")]
pub fn set(body: Json<Snapshot>, state: State<SelectedSnapshot>) -> YyxResult<()> {
  save_last_snapshot(&body)?;
  state.set(body.into_inner());
  Ok(())
}
