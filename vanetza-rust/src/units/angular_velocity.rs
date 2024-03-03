use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use uom::si::{angular_velocity::radian_per_second, f64 as si};
use vanetza_sys::vanetza_wrapper::units::AngularVelocityWrapper as CxxAngularVelocity;

pub type AngularVelocity = si::AngularVelocity;

impl ToCxxUniquePtr<CxxAngularVelocity> for AngularVelocity {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxAngularVelocity> {
        let value = self.get::<radian_per_second>();
        CxxAngularVelocity::new1(value).within_unique_ptr()
    }
}

impl FromCxxRef<CxxAngularVelocity> for AngularVelocity {
    fn from_cxx_ref(src: &CxxAngularVelocity) -> Self {
        AngularVelocity::new::<radian_per_second>(src.as_radians_per_second())
    }
}
