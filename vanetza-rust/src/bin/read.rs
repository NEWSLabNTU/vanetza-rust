use autocxx::c_void;
use cxx::CxxString;
use cxx::UniquePtr;
use vanetza_sys::btpb_write;
use vanetza_sys::btpb_read;
use cxx::let_cxx_string;
use std::pin::Pin;
use cxx::CxxVector;
fn main(){
    let binding: UniquePtr<CxxVector<u8>> = CxxVector::new();
    let mut buf: *mut CxxVector<u8> = binding.into_raw();

    let binding = unsafe {
        btpb_read("lo", buf);
        UniquePtr::from_raw(buf)
    };
    println!("{:?}",binding);
}