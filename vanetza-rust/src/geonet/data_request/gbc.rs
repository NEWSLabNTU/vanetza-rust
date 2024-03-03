use super::{DataRequest, DataRequestWithArea};
use crate::{
    geonet::{Area, MIB},
    ItsAid,
};
use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::{
    DataRequest as CxxDataRequest, DataRequestWithArea as CxxDataRequestWithArea,
    GbcDataRequest as CxxGbcDataRequest,
};

pub struct GbcDataRequest {
    pub(crate) ptr: UniquePtr<CxxGbcDataRequest>,
}

impl DataRequest for GbcDataRequest {
    fn as_cxx_ref(&self) -> &CxxDataRequest {
        todo!()
    }
}

impl DataRequestWithArea for GbcDataRequest {
    fn as_cxx_ref(&self) -> &CxxDataRequestWithArea {
        todo!()
    }
}

impl GbcDataRequest {
    pub fn new(mib: &MIB, its_aid: ItsAid, area: Area) -> Self {
        todo!();
    }
}
