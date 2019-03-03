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

mod helpers;
mod logger;
mod result;
mod routes;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 1128;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
  "OK"
}

fn main() {
  env::set_var("ROCKET_CLI_COLORS", "off");

  logger::setup_logger().unwrap();
  yyx_data::init().unwrap();

  ping_and_launch_browser();

  let config = Config::build(Environment::Production)
    .address(HOST)
    .port(PORT)
    .secret_key("8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=")
    .limits(Limits::new().limit("json", 100 * 1024 * 1024))
    .unwrap();
  rocket::custom(config)
    .manage(helpers::SelectedSnapshot::load())
    .mount("/", routes![routes::app::static_file, index])
    .mount(
      "/api",
      routes![
        ping,
        routes::snapshot::set,
        routes::snapshot::get,
        routes::equip::list,
        routes::hero::list,
      ],
    )
    .launch();
}

fn ping_and_launch_browser() {
  use reqwest;
  use std::thread;
  use std::time::Duration;
  let url = format!("http://{}:{}", HOST, PORT);
  let ping_url = format!("{}/api/ping", url);
  thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_secs(1));
      if reqwest::get(&ping_url).is_ok() {
        #[cfg(not(debug_assertions))]
        launch_browser();
        break;
      }
    }
    println!("痒痒熊已经启动: {}", url);
  });
}

#[cfg(target_os = "windows")]
fn launch_browser() {
  use std::process::Command;
  Command::new("start")
    .args(&[&format!("http://{}:{}", HOST, PORT)])
    .spawn()
    .unwrap();
}

#[cfg(target_os = "macos")]
fn launch_browser() {
  use std::process::Command;
  Command::new("open")
    .args(&[&format!("http://{}:{}", HOST, PORT)])
    .spawn()
    .unwrap();
}

#[cfg(target_os = "linux")]
fn launch_browser() {
  use std::process::Command;
  Command::new("xdg-open")
    .args(&[&format!("http://{}:{}", HOST, PORT)])
    .spawn()
    .unwrap();
}
