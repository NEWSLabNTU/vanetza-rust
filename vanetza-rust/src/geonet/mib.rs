use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::ManagementInformationBase as CxxMIB;

// enums
pub use vanetza_sys::vanetza::geonet::AddrConfMethod;
pub use vanetza_sys::vanetza::geonet::BroadcastForwarding;
pub use vanetza_sys::vanetza::geonet::InterfaceType;
pub use vanetza_sys::vanetza::geonet::SecurityDecapHandling;
pub use vanetza_sys::vanetza::geonet::UnicastForwarding;

// alias
pub type MIB = ManagementInformationBase;

pub struct ManagementInformationBase {
    // TODO
}

impl ManagementInformationBase {
    pub(crate) fn to_cxx_ptr(&self) -> UniquePtr<CxxMIB> {
        todo!();
    }
}
