use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use uom::si::{f64 as si, frequency::hertz};
use vanetza_sys::vanetza_wrapper::units::FrequencyWrapper as CxxFrequency;

pub type Frequency = si::Frequency;

impl ToCxxUniquePtr<CxxFrequency> for Frequency {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxFrequency> {
        let value = self.get::<hertz>();
        CxxFrequency::new1(value).within_unique_ptr()
    }
}

impl FromCxxRef<CxxFrequency> for Frequency {
    fn from_cxx_ref(src: &CxxFrequency) -> Self {
        Frequency::new::<hertz>(src.as_hertz())
    }
}
