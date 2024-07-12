use autocxx::c_void;
use cxx::{let_cxx_string, CxxString, CxxVector, UniquePtr};
use std::pin::Pin;
use vanetza_sys::{btpb_read, btpb_write};

fn main() {
    let binding: UniquePtr<CxxVector<u8>> = CxxVector::new();
    let mut buf: *mut CxxVector<u8> = binding.into_raw();

    let binding = unsafe {
        btpb_read("lo", buf);
        UniquePtr::from_raw(buf)
    };
    println!("{:?}", binding);
}
