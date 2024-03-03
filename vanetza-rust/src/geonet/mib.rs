use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::ManagementInformationBase as CxxMIB;

// enums
pub use vanetza_sys::vanetza::geonet::{
    AddrConfMethod, BroadcastForwarding, InterfaceType, SecurityDecapHandling, UnicastForwarding,
};

// alias
pub type MIB = ManagementInformationBase;

pub struct ManagementInformationBase {
    // TODO
}

impl ManagementInformationBase {
    pub(crate) fn to_cxx_ptr(&self) -> UniquePtr<CxxMIB> {
        todo!();
    }

    pub(crate) fn from_cxx(mib: &CxxMIB) -> Self {
        todo!();
    }
}
