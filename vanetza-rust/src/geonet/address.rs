use crate::common::{CStationType, StationType};
use mac_address::MacAddress;
use vanetza_sys as sys;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub manually_configured: bool,
    pub station_type: StationType,
    pub country_code: u16,
    pub mid: MacAddress,
}

pub(crate) struct CAddress(pub(crate) sys::c_address);

impl CAddress {
    pub fn manually_configured(&self) -> bool {
        unsafe { sys::address_get_is_manually_configured(self.0) }
    }

    pub fn station_type(&self) -> StationType {
        unsafe {
            let val = sys::address_get_station_type(self.0);
            CStationType(val)
                .try_into()
                .unwrap_or_else(|_| panic!("Unexpected station type code {val}"))
        }
    }

    pub fn country_code(&self) -> u16 {
        unsafe { sys::address_get_country_code(self.0) }
    }

    pub fn mid(&self) -> MacAddress {
        unsafe {
            let val = sys::address_get_mid(self.0);
            u64_to_mac(val)
        }
    }
}

impl From<CAddress> for Address {
    fn from(addr: CAddress) -> Self {
        Self {
            manually_configured: addr.manually_configured(),
            station_type: addr.station_type(),
            country_code: addr.country_code(),
            mid: addr.mid(),
        }
    }
}

impl From<MacAddress> for CAddress {
    fn from(addr: MacAddress) -> Self {
        unsafe {
            let ptr = sys::address_new1(mac_to_u64(addr));
            Self(ptr)
        }
    }
}

impl From<Address> for CAddress {
    fn from(addr: Address) -> Self {
        let Address {
            manually_configured,
            station_type,
            country_code,
            mid,
        } = addr;

        unsafe {
            let ptr = sys::address_new2(
                manually_configured,
                station_type as u16,
                country_code,
                mac_to_u64(mid),
            );
            Self(ptr)
        }
    }
}

fn mac_to_u64(mid: MacAddress) -> u64 {
    let mut bytes = [0u8; 8];
    mid.bytes()
        .iter()
        .zip(bytes.iter_mut().rev())
        .for_each(|(&src, dst)| {
            *dst = src;
        });
    u64::from_ne_bytes(bytes)
}

fn u64_to_mac(val: u64) -> MacAddress {
    let mut bytes = [0u8; 6];

    bytes
        .iter_mut()
        .zip(val.to_ne_bytes()[2..8].iter().rev())
        .for_each(|(dst, &src)| *dst = src);

    bytes.into()
}
