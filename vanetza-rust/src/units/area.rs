use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use uom::si::{area::square_meter, f64 as si};
use vanetza_sys::vanetza_wrapper::units::AreaWrapper as CxxArea;

pub type Area = si::Area;

impl ToCxxUniquePtr<CxxArea> for Area {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxArea> {
        let value = self.get::<square_meter>();
        CxxArea::new1(value).within_unique_ptr()
    }
}

impl FromCxxRef<CxxArea> for Area {
    fn from_cxx_ref(src: &CxxArea) -> Self {
        Area::new::<square_meter>(src.as_square_meters())
    }
}
