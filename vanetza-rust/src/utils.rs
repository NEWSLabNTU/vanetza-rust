use mac_address::MacAddress;
use vanetza_sys::vanetza::MacAddress as CxxMacAddress;

pub(crate) trait MacAddressExt {
    fn to_cxx(&self) -> CxxMacAddress;
    fn from_cxx(addr: CxxMacAddress) -> Self;
}

impl MacAddressExt for MacAddress {
    fn to_cxx(&self) -> CxxMacAddress {
        CxxMacAddress {
            _base: 0,
            octets: self.bytes(),
        }
    }

    fn from_cxx(addr: CxxMacAddress) -> Self {
        Self::new(addr.octets)
    }
}
