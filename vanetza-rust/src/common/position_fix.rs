use cxx::UniquePtr;
use vanetza_sys::vanetza::PositionFix as CxxPositionFix;

pub struct PositionFix {
    // TODO
}

impl PositionFix {
    pub fn to_cxx_ptr(&self) -> UniquePtr<CxxPositionFix> {
        todo!();
    }
}
