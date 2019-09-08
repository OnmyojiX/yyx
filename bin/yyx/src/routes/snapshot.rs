use futures_fs::FsPool;
use serde_derive::Deserialize;
use warp::{http::Response, path, Filter, Rejection, Reply};

use crate::helpers::*;
use crate::result::*;
use crate::store::{AccountStateRef, YyxStoreRef};

use yyx_cbg::result::CbgError;
use yyx_data::{get_last_snapshot_path, save_last_snapshot, AccountId};
use yyx_types::Snapshot;

pub fn account_id() -> impl Filter<Extract = (AccountId,), Error = Rejection> + Clone {
  path!("yyx" / i64 / i64)
    .and_then(|server_id, player_id| -> Result<_, Rejection> {
      Ok(AccountId::Yyx {
        server_id,
        player_id,
      })
    })
    .or(
      path!("cbg" / String / String).and_then(|server_id, order_sn| -> Result<_, Rejection> {
        Ok(AccountId::Cbg {
          server_id,
          order_sn,
        })
      }),
    )
    .unify()
    .or(
      warp::any().and_then(|| Err::<_, Rejection>(YyxError::bad_request("无法识别账户ID").into())),
    )
    .unify()
}

pub fn account_state(
  store: YyxStoreRef,
) -> impl Filter<Extract = (AccountStateRef,), Error = Rejection> + Clone {
  account_id().and_then(move |id| {
    store
      .get_account(&id)
      .ok_or_else(|| -> Rejection { YyxError::bad_request("账户不存在").into() })
  })
}

pub fn get(store: YyxStoreRef) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::get2()
    .and(account_state(store))
    .and(path("snapshot"))
    .map(|r: AccountStateRef| {
      let state = r.read().unwrap();
      warp::reply::json(&state.snapshot as &Snapshot)
    })
}

pub fn export(
  store: YyxStoreRef,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::get2()
    .and(account_state(store))
    .and(path!("snapshot-export" / String))
    .and_then(|_state, name: String| -> Result<_, Rejection> {
      use hyper::Body;
      let path = get_last_snapshot_path().map_err(|err| -> Rejection {
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
  store: YyxStoreRef,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::put2()
    .and(path("snapshot-cbg"))
    .and(warp::body::json())
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

pub fn set(store: YyxStoreRef) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
  warp::put2()
    .and(warp::filters::body::content_length_limit(20 * 1024 * 1024))
    .and(warp::body::json())
    .and_then(move |snapshot: Snapshot| {
      let id = if let Some(cbg_url) = snapshot.cbg_url.as_ref() {
        match yyx_cbg::get_params_from_url(cbg_url) {
          Ok(params) => AccountId::Cbg {
            server_id: params.server_id,
            order_sn: params.order_sn,
          },
          Err(err) => {
            return Err(
              YyxError::bad_request(format!("解析藏宝阁参数错误: {}", err)).into_rejection(),
            )
          }
        }
      } else {
        AccountId::Yyx {
          server_id: snapshot.data.player.server_id,
          player_id: snapshot.data.player.id,
        }
      };
      store.set_account(id.clone(), snapshot);
      Ok(warp::reply::json(&id))
    })
}
