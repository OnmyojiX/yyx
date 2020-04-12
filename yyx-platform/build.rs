#[cfg(target_os = "macos")]
extern crate cc;

#[cfg(target_os = "macos")]
fn main() {
  cc::Build::new().file("src/macos/proc.c").compile("proc");
}

#[cfg(not(target_os = "macos"))]
fn main() {}
