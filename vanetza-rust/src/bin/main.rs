use cxx::CxxString;
use vanetza_sys::btpb_write;
use vanetza_sys::btpb_read;
use cxx::let_cxx_string;
use std::pin::Pin;
fn main(){
    let_cxx_string!(buf = "");
    //let buf_ref = buf.as_ref();
    let a = btpb_read("lo",buf.as_mut());
    println!("{:?}",buf);
}