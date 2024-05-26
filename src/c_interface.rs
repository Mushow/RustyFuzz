use libc::c_char;
use std::ffi::CString;

extern "C" {
    fn vulnerable_function(input: *const c_char);
}

pub fn call_vulnerable_function(input: &str) {
    let c_input = CString::new(input).expect("CString::new failed");
    unsafe {
        vulnerable_function(c_input.as_ptr());
    }
}
