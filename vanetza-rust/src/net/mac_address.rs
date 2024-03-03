use crate::utils::{FromCxxRef, ToCxxPod};
use mac_address::MacAddress;
use vanetza_sys::vanetza::MacAddress as CxxMacAddress;

impl ToCxxPod<CxxMacAddress> for MacAddress {
    fn to_cxx_pod(&self) -> CxxMacAddress {
        CxxMacAddress {
            _base: 0,
            octets: self.bytes(),
        }
    }
}

impl FromCxxRef<CxxMacAddress> for MacAddress {
    fn from_cxx_ref(src: &CxxMacAddress) -> Self {
        Self::new(src.octets)
    }
}
