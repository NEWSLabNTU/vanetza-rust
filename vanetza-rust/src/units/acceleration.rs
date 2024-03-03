use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use uom::si::{acceleration::meter_per_second_squared, f64 as si};
use vanetza_sys::vanetza_wrapper::units::AccelerationWrapper as CxxAcceleration;

pub type Acceleration = si::Acceleration;

impl ToCxxUniquePtr<CxxAcceleration> for Acceleration {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxAcceleration> {
        let meters = self.get::<meter_per_second_squared>();
        CxxAcceleration::new1(meters).within_unique_ptr()
    }
}

impl FromCxxRef<CxxAcceleration> for Acceleration {
    fn from_cxx_ref(src: &CxxAcceleration) -> Self {
        Acceleration::new::<meter_per_second_squared>(src.as_meters_per_second_squared())
    }
}
