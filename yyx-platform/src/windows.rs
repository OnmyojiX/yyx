pub fn setup_env() {}

pub fn launch_browser(url: &str) {
  use std::ffi::CString;
  use std::ptr;
  use winapi::um::shellapi::ShellExecuteA;
  let cstr = CString::new(url).unwrap();
  unsafe {
    ShellExecuteA(
      ptr::null_mut(),
      ptr::null_mut(),
      cstr.as_ptr(),
      ptr::null_mut(),
      ptr::null_mut(),
      1,
    );
  }
}
