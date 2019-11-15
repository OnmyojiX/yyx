use futures_fs::FsPool;
use serde_derive::Deserialize;
use warp::{http::Response, path, Filter, Rejection, Reply};

use yyx_cbg::result::CbgError;
use yyx_data::{
  db::player, get_last_snapshot_path, load_last_snapshot, save_last_snapshot, AccountId, DbRef,
};
use yyx_types::Snapshot;

use crate::helpers::*;
use crate::result::*;
use crate::routes::account::account_id;
use crate::store::YyxStoreRef;

pub fn get(store: YyxStoreRef) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::get2()
    .and(account_id())
    .and(path("snapshot"))
    .and_then(move |id: AccountId| {
      let store = store.clone();
      block(move || {
        if let Some(r) = store.get_account(&id) {
          let state = r.read().unwrap();
          Ok(warp::reply::json(&state.snapshot as &Snapshot))
        } else {
          let s = load_last_snapshot(&id).map_err(|err| {
            YyxError::internal(format!("读取快照文件失败: {}", err)).into_rejection()
          })?;
          if let Some(s) = s {
            let id = inspect_snapshot(&s).map_err(YyxError::into_rejection)?;
            let reply = warp::reply::json(&s);
            store.open_account(id, s);
            Ok(reply)
          } else {
            Err(YyxError::bad_request("账号不存在").into_rejection())
          }
        }
      })
    })
}

pub fn export(
  _store: YyxStoreRef,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::get2()
    .and(account_id())
    .and(path!("snapshot-export" / String))
    .and_then(|id: AccountId, name: String| -> Result<_, Rejection> {
      use hyper::Body;
      let path = get_last_snapshot_path(&id).map_err(|err| -> Rejection {
        YyxError::bad_request(format!("读取快照文件路径错误: {}", err)).into()
      })?;
      let stream = FsPool::default().read(path, Default::default());
      Ok::<_, Rejection>(attachment(
        Response::builder()
          .body(Body::wrap_stream(stream))
          .map_err(|err| YyxError::internal(err).into_rejection())?,
        &name,
      ))
    })
}

#[derive(Debug, Deserialize)]
pub struct PullCbg {
  url: String,
}

pub fn pull_cbg(
  _store: YyxStoreRef,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::put2()
    .and(path("snapshot-cbg"))
    .and(warp::body::json::<PullCbg>())
    .and_then(|body: PullCbg| {
      block(move || -> Result<_, Rejection> {
        let snapshot = yyx_cbg::pull::pull(&body.url, crate::version::VERSION).map_err(|err| {
          match err {
            CbgError::UrlPrefix(url) => {
              YyxError::bad_request(format!("藏宝阁网址应该以'{}'开头。", url))
            }
            CbgError::InvalidUrl => YyxError::bad_request(format!("藏宝阁网址错误。")),
            err => YyxError::internal(format!("读取藏宝阁数据出错: {:?}", err)),
          }
          .into_rejection()
        })?;
        Ok(warp::reply::json(&snapshot))
      })
    })
}

pub fn import(
  store: YyxStoreRef,
  db: DbRef,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::post2()
    .and(path("import"))
    .and(warp::filters::body::content_length_limit(50 * 1024 * 1024))
    .and(warp::body::json())
    .and_then(move |snapshot: Snapshot| {
      let store = store.clone();
      let db = db.clone();
      block(move || -> Result<_, Rejection> {
        let id = inspect_snapshot(&snapshot).map_err(YyxError::into_rejection)?;
        save_last_snapshot(&id, &snapshot).map_err(|err| {
          YyxError::internal(format!("导入快照文件失败: {}", err)).into_rejection()
        })?;

        let p = db
          .exec({
            let info = player::SnapshotInfo::from(&snapshot);
            let id = id.clone();
            move |conn| player::upsert(conn, &id, &info)
          })
          .map_err(|err| {
            YyxError::internal(format!("导入快照记录失败: {}", err)).into_rejection()
          })?;
        store.open_account(id.clone(), snapshot);
        Ok(warp::reply::json(&p))
      })
    })
}

fn inspect_snapshot(snapshot: &Snapshot) -> Result<AccountId, YyxError> {
  let id = if let Some(cbg_url) = snapshot.cbg_url.as_ref() {
    match yyx_cbg::get_params_from_url(cbg_url) {
      Ok(params) => AccountId::Cbg {
        server_id: params.server_id,
        order_sn: params.order_sn,
      },
      Err(err) => {
        return Err(YyxError::bad_request(format!(
          "解析藏宝阁参数错误: {}",
          err
        )))
      }
    }
  } else {
    AccountId::Yyx {
      server_id: snapshot.data.player.server_id,
      player_id: snapshot.data.player.id,
    }
  };
  Ok(id)
}
