#![warn(clippy::all)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::config::{Config, Environment, Limits};
use std::env;

use yyx_config::YyxConfig;

mod helpers;
mod logger;
mod result;
mod routes;

#[get("/ping")]
fn ping() -> &'static str {
  "OK"
}

fn main() {
  yyx_platform::setup_env();
  env::set_var("ROCKET_CLI_COLORS", "off");

  logger::setup_logger().unwrap();
  yyx_data::init().unwrap();

  let config = yyx_config::read_or_create_default();

  ping_and_launch_browser(&config);

  let config = Config::build(Environment::Production)
    .address(&config.host as &str)
    .port(config.port)
    .secret_key("8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=")
    .limits(Limits::new().limit("json", 100 * 1024 * 1024))
    .unwrap();
  rocket::custom(config)
    .manage(helpers::SelectedSnapshot::load())
    .mount(
      "/",
      routes![
        routes::app::static_file,
        routes::app::index,
        routes::export::files
      ],
    )
    .mount(
      "/api",
      routes![
        ping,
        routes::snapshot::set,
        routes::snapshot::get,
        routes::equip::list,
        routes::hero::list,
        routes::export::export_json
      ],
    )
    .launch();
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
