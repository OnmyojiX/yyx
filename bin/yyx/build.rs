#[cfg(target_os = "macos")]
extern crate cc;

#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
  let mut res = winres::WindowsResource::new();
  res.set_icon("yyx.ico");
  res.compile().unwrap();
}

#[cfg(target_os = "macos")]
fn main() {
  cc::Build::new()
    .file("src/macos/proc.c")
    .compile("proc");
}
