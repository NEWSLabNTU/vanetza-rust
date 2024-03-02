use super::mib::MIB;
use crate::common::Runtime;
use vanetza_sys as sys;

pub struct Router(pub(crate) sys::c_router);

impl Router {
    pub fn new(runtime: &impl Runtime, mib: &MIB) -> Self {
        unsafe { Self(sys::router_new(runtime.to_raw_ptr(), mib.0)) }
    }
}

impl Drop for Router {
    fn drop(&mut self) {
        unsafe {
            sys::router_del(self.0);
        }
    }
}
