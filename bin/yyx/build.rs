#[cfg(target_os = "macos")]
extern crate cc;

#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
  let mut res = winres::WindowsResource::new();
  res.set_icon("yyx.ico");
  res.compile().unwrap();
  write_version()
}

#[cfg(target_os = "macos")]
fn main() {
  cc::Build::new().file("src/macos/proc.c").compile("proc");
  write_version()
}

#[cfg(target_os = "linux")]
fn main() {
  write_version()
}

fn write_version() {
  use std::{env, fs};
  let verison_file_path = format!("{}/version.rs", env::var("OUT_DIR").unwrap());
  let version = env::var("CARGO_PKG_VERSION").unwrap();
  fs::write(
    verison_file_path,
    format!(r#"pub const VERSION: &str = "{}";"#, version),
  )
  .unwrap();
}
