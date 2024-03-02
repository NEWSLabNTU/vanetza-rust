use cxx::UniquePtr;
use std::pin::Pin;

pub use vanetza_sys::vanetza::geonet::DataRequest as CxxDataRequest;
pub use vanetza_sys::vanetza::geonet::DataRequestWithAddress as CxxDataRequestWithAddress;
pub use vanetza_sys::vanetza::geonet::DataRequestWithArea as CxxDataRequestWithArea;
pub use vanetza_sys::vanetza::geonet::GacDataRequest as CxxGacDataRequest;
pub use vanetza_sys::vanetza::geonet::GbcDataRequest as CxxGbcDataRequest;
pub use vanetza_sys::vanetza::geonet::GucDataRequest as CxxGucDataRequest;
pub use vanetza_sys::vanetza::geonet::ShbDataRequest as CxxShbDataRequest;
pub use vanetza_sys::vanetza::geonet::TsbDataRequest as CxxTsbDataRequest;

pub trait DataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequest>;
}

pub trait DataRequestWithArea: DataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequestWithArea>;
}

pub trait DataRequestWithAddress: DataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequestWithAddress>;
}

pub struct GucDataRequest {
    ptr: UniquePtr<CxxGucDataRequest>,
}

pub struct GacDataRequest {
    ptr: UniquePtr<CxxGacDataRequest>,
}

pub struct GbcDataRequest {
    ptr: UniquePtr<CxxGbcDataRequest>,
}

pub struct ShbDataRequest {
    ptr: UniquePtr<CxxShbDataRequest>,
}

pub struct TsbDataRequest {
    ptr: UniquePtr<CxxTsbDataRequest>,
}

impl DataRequest for GucDataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequest> {
        todo!()
    }
}

impl DataRequestWithAddress for GucDataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequestWithAddress> {
        todo!()
    }
}

impl DataRequest for GbcDataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequest> {
        todo!()
    }
}

impl DataRequestWithArea for GbcDataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequestWithArea> {
        todo!()
    }
}

impl DataRequest for GacDataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequest> {
        todo!()
    }
}

impl DataRequestWithArea for GacDataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequestWithArea> {
        todo!()
    }
}

impl DataRequest for ShbDataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequest> {
        todo!()
    }
}

impl DataRequest for TsbDataRequest {
    fn as_cxx_pin_mut(&mut self) -> Pin<&mut CxxDataRequest> {
        todo!()
    }
}
