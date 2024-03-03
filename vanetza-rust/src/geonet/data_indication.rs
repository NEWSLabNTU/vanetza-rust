use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::DataIndication as CxxDataIndication;

pub struct DataIndication {
    ptr: UniquePtr<CxxDataIndication>,
}
