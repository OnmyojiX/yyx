use serde_derive::{Deserialize, Serialize};

/// 式神召唤书碎片
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HeroBookShard {
  /// 式神类型ID
  pub hero_id: i64,
  /// 碎片数量
  pub shards: i64,
  /// 召唤书数量
  pub books: i64,
  /// 召唤书最大碎片数量
  pub book_max_shards: i64,
}
