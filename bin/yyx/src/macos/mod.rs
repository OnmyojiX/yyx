#![allow(unused)]

use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::ptr;

#[link(name = "proc")]
extern "C" {
  fn get_self_path() -> *const c_char;
}

pub fn get_self_dir() -> PathBuf {
  use std::ffi::CStr;
  let p = unsafe { get_self_path() };
  if p == ptr::null() {
    panic!("Get self dir failed.");
  }
  let cstr = unsafe { CStr::from_ptr(p) };
  let s = cstr.to_string_lossy().to_string();
  Path::new(&s).parent().unwrap().to_owned()
}
