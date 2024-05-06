use std::os::raw::{c_char};
use std::ffi::CString;

#[no_mangle]
pub extern fn hello() -> *mut c_char {
    CString::new("Hello from Rust!").unwrap().into_raw()
}

#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}
