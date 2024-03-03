use vanetza_sys::vanetza::geonet::{
    LongPositionVector as CxxLongPositionVector, ShortPositionVector as CxxShortPositionVector,
};

pub struct LongPositionVectorRef<'a> {
    pub(crate) ref_: &'a CxxLongPositionVector,
}

pub struct ShortPositionVectorRef<'a> {
    pub(crate) ref_: &'a CxxShortPositionVector,
}
