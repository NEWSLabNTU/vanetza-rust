use crate::{geonet::StationType, utils::ToCxxPod};
use autocxx::prelude::*;
use cxx::UniquePtr;
use mac_address::MacAddress;
use vanetza_sys::vanetza::geonet::Address as CxxAddress;

pub struct Address {
    pub manually_configured: bool,
    pub station_type: StationType,
    pub country_code: u16,
    pub mid: MacAddress,
}

impl Address {
    pub fn to_cxx_ptr(&self) -> UniquePtr<CxxAddress> {
        let Self {
            manually_configured,
            station_type,
            country_code,
            mid,
        } = self;

        let mut ptr = CxxAddress::new().within_unique_ptr();
        ptr.pin_mut().is_manually_configured1(*manually_configured);
        ptr.pin_mut().station_type1(station_type.clone());
        ptr.pin_mut().mid1(&mid.to_cxx_pod());
        todo!("set country_code");

        ptr
    }
}
