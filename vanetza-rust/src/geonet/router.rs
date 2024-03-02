use super::mib::MIB;
use crate::Runtime;
use autocxx::prelude::*;
use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::Router as CxxRouter;

pub struct Router {
    ptr: UniquePtr<CxxRouter>,
}

impl Router {
    pub fn new(runtime: &impl Runtime, mib: &MIB) -> Self {
        let ptr = CxxRouter::new(runtime.as_cxx_pin_mut(), &mib.to_cxx_ptr()).within_unique_ptr();
        Self { ptr }
    }
}
