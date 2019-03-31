pub fn setup_env() {}

pub fn launch_browser(url: &str) {
  use std::process::Command;
  Command::new("xdg-open")
    .args(&[url])
    .spawn()
    .unwrap();
}
