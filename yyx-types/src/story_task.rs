use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StoryTask {
  pub id: i64,
  pub progress: StoryTaskProgress,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StoryTaskProgress {
  pub value: i64,
  pub max_value: i64,
}
