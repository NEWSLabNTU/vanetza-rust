use cxx::UniquePtr;
use vanetza_sys::vanetza::geonet::TransportInterface as CxxTransportInterface;

pub struct TransportInterface {
    ptr: UniquePtr<CxxTransportInterface>,
}
