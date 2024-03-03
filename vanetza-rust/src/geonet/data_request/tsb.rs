use super::DataRequest;
use crate::{geonet::MIB, ItsAid};
use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::{
    // DataRequest as CxxDataRequest,
    TsbDataRequest as CxxTsbDataRequest,
};

pub struct TsbDataRequest {
    pub(crate) ptr: UniquePtr<CxxTsbDataRequest>,
}

impl DataRequest for TsbDataRequest {
    // fn as_cxx_ref(&self) -> &CxxDataRequest {
    //     todo!()
    // }
}

impl TsbDataRequest {
    pub fn new(mib: &MIB, its_aid: ItsAid) -> Self {
        todo!();
    }
}
