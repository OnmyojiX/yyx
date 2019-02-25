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

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

fn main() {
  env::set_var("ROCKET_CLI_COLORS", "off");

  logger::setup_logger().unwrap();
  yyx_data::init().unwrap();

  let config = Config::build(Environment::Production)
    .address("127.0.0.1")
    .port(1128)
    .secret_key("8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=")
    .limits(Limits::new().limit("json", 100 * 1024 * 1024))
    .unwrap();
  rocket::custom(config)
    .manage(helpers::SelectedSnapshot::new())
    .mount("/", routes![index])
    .mount("/api", routes![routes::snapshot::set, routes::equip::list,])
    .launch();
}
