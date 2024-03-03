use cxx::UniquePtr;
use vanetza_sys::vanetza::CohesivePacket as CxxCohesivePacket;

pub struct CohesivePacket {
    pub(crate) ptr: UniquePtr<CxxCohesivePacket>,
}
