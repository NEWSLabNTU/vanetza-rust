use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use uom::si::{f64 as si, length::meter};
use vanetza_sys::vanetza_wrapper::units::LengthWrapper as CxxLength;

pub type Length = si::Length;

impl ToCxxUniquePtr<CxxLength> for Length {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxLength> {
        let meters = self.get::<meter>();
        CxxLength::new1(meters).within_unique_ptr()
    }
}

impl FromCxxRef<CxxLength> for Length {
    fn from_cxx_ref(src: &CxxLength) -> Self {
        Length::new::<meter>(src.as_meters())
    }
}
