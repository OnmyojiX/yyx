pub fn setup_env() {}

fn launch_browser() {
  use std::process::Command;
  Command::new("xdg-open")
    .args(&[&format!("http://{}:{}", HOST, PORT)])
    .spawn()
    .unwrap();
}
