use autocxx::c_void;
use cxx::{let_cxx_string, CxxString};
use std::pin::Pin;
use vanetza_sys::{btpb_read, btpb_write};
fn main() {
    let binding = String::from("newslab");
    let binding = binding.as_bytes();
    let size = binding.len();
    let ptr = binding.as_ptr();
    let buf: *mut c_void = ptr as *mut c_void;
    unsafe {
        btpb_write("lo", buf, size);
    }
}
