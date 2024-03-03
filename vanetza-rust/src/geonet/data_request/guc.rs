use super::{DataRequest, DataRequestWithAddress};
use crate::{
    geonet::{Address, MIB},
    ItsAid,
};
use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::{
    // DataRequest as CxxDataRequest, DataRequestWithAddress as CxxDataRequestWithAddress,
    GucDataRequest as CxxGucDataRequest,
};

pub struct GucDataRequest {
    pub(crate) ptr: UniquePtr<CxxGucDataRequest>,
}

impl DataRequest for GucDataRequest {
    // fn as_cxx_ref(&self) -> &CxxDataRequest {
    //     todo!()
    // }
}

impl DataRequestWithAddress for GucDataRequest {
    // fn as_cxx_ref(&self) -> &CxxDataRequestWithAddress {
    //     todo!()
    // }
}

impl GucDataRequest {
    pub fn new(mib: &MIB, its_aid: ItsAid, address: Address) -> Self {
        todo!();
    }
}
