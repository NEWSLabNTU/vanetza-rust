use super::{DataRequest, DataRequestWithArea};
use crate::{
    geonet::{Area, MIB},
    ItsAid,
};
use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::{
    // DataRequest as CxxDataRequest, DataRequestWithArea as CxxDataRequestWithArea,
    GacDataRequest as CxxGacDataRequest,
};

pub struct GacDataRequest {
    pub(crate) ptr: UniquePtr<CxxGacDataRequest>,
}

impl DataRequest for GacDataRequest {
    // fn as_cxx_ref(&self) -> &CxxDataRequest {
    //     todo!()
    // }
}

impl DataRequestWithArea for GacDataRequest {
    // fn as_cxx_ref(&self) -> &CxxDataRequestWithArea {
    //     todo!()
    // }
}

impl GacDataRequest {
    pub fn new(mib: &MIB, its_aid: ItsAid, area: Area) -> Self {
        todo!();
    }
}
