use autocxx::c_void;
use cxx::CxxString;
use vanetza_sys::btpb_write;
use vanetza_sys::btpb_read;
use cxx::let_cxx_string;
use std::pin::Pin;
fn main(){
    let binding = String::from("newslab");
    let binding = binding.as_bytes();
    let size = binding.len();
    let ptr = binding.as_ptr();
    let buf :*mut c_void = ptr as *mut c_void;
    unsafe {
        btpb_write("lo",buf,size);
    }
}