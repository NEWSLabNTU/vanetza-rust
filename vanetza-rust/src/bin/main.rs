use cxx::CxxString;
use vanetza_sys::btpb_write;
use vanetza_sys::btpb_read;
use cxx::let_cxx_string;
use std::pin::Pin;
fn main(){
    let_cxx_string!(buf = "");
    btpb_read("lo",buf.as_mut());
    btpb_write("lo",b"newslab".as_slice());
}