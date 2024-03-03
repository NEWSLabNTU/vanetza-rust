use super::DataRequest;
use crate::{geonet::MIB, ItsAid};
use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::{
    DataRequest as CxxDataRequest, ShbDataRequest as CxxShbDataRequest,
};

pub struct ShbDataRequest {
    pub(crate) ptr: UniquePtr<CxxShbDataRequest>,
}

impl DataRequest for ShbDataRequest {
    fn as_cxx_ref(&self) -> &CxxDataRequest {
        todo!()
    }
}

impl ShbDataRequest {
    pub fn new(mib: &MIB, its_aid: ItsAid) -> Self {
        todo!();
    }
}
