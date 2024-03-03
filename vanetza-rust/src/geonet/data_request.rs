mod gac;
mod gbc;
mod guc;
mod shb;
mod tsb;
mod variant;

pub use gac::*;
pub use gbc::*;
pub use guc::*;
pub use shb::*;
pub use tsb::*;
pub use variant::*;

pub trait DataRequest {
    // fn as_cxx_ref(&self) -> &CxxDataRequest;
}

pub trait DataRequestWithArea: DataRequest {
    // fn as_cxx_ref(&self) -> &CxxDataRequestWithArea;
}

pub trait DataRequestWithAddress: DataRequest {
    // fn as_cxx_ref(&self) -> &CxxDataRequestWithAddress;
}
