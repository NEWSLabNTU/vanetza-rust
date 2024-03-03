use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use uom::si::{f64 as si, velocity::meter_per_second};
use vanetza_sys::vanetza_wrapper::units::VelocityWrapper as CxxVelocity;

pub type Velocity = si::Velocity;

impl ToCxxUniquePtr<CxxVelocity> for Velocity {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxVelocity> {
        let value = self.get::<meter_per_second>();
        CxxVelocity::new1(value).within_unique_ptr()
    }
}

impl FromCxxRef<CxxVelocity> for Velocity {
    fn from_cxx_ref(src: &CxxVelocity) -> Self {
        Velocity::new::<meter_per_second>(src.as_meters_per_second())
    }
}
