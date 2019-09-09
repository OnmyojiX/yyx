#![warn(clippy::all)]

use futures01::sync::oneshot;

use warp::{path, Filter};

use yyx_config::YyxConfig;
use yyx_data::DbRef;

mod helpers;
mod logger;
mod result;
mod routes;
mod store;
mod version;

use self::store::YyxStore;

fn main() {
  logger::setup_logger().expect("初始化日志系统失败");
  yyx_data::init().expect("初始化数据文件夹失败");

  let config = yyx_config::read_or_create_default();
  let db = DbRef::new().expect("初始化数据库失败");

  ping_and_launch_browser(&config);

  let ping = path!("ping")
    .and(warp::path::end())
    .map(|| version::VERSION);

  let store = YyxStore::new_ref();

  let routes = path("api")
    .and(
      ping
        .or(routes::account::get(db.clone()))
        .or(routes::account::list(db.clone()))
        .or(routes::account::list_active(store.clone()))
        .or(routes::account::close(store.clone()))
        .or(routes::account::delete(db.clone()))
        .or(routes::snapshot::get(store.clone()))
        .or(routes::snapshot::import(store.clone(), db.clone()))
        .or(routes::snapshot::export(store.clone()))
        .or(routes::snapshot::pull_cbg(store.clone()))
        .or(routes::export::export_json())
        .map(|reply| warp::reply::with_header(reply, "Cache-Control", "no-cache"))
        .recover(result::handle_rejection),
    )
    .or(routes::app::static_files().or(routes::export::files()))
    .or_else(|_| Err(warp::reject::not_found()))
    .with(warp::log("yyx"));

  let addr = format!("{}:{}", config.host, config.port)
    .parse::<std::net::SocketAddr>()
    .expect(&format!("无效的地址: {}:{}", config.host, config.port));

  let (_shutdown, rx) = oneshot::channel();
  let (_addr, server) = warp::serve(routes).bind_with_graceful_shutdown(addr, rx);
  tokio::run(server);
}

fn ping_and_launch_browser(config: &YyxConfig) {
  use std::thread;
  use std::time::Duration;
  let url = format!("http://{}:{}", config.host, config.port);
  let ping_url = format!("{}/api/ping", url);
  thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_secs(1));
      if reqwest::get(&ping_url).is_ok() {
        #[cfg(not(debug_assertions))]
        yyx_platform::launch_browser(&url);
        break;
      }
    }
    println!("痒痒熊已经启动: {}", url);
  });
}
