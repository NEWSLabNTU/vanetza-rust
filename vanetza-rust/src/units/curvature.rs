use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use uom::si::{curvature::radian_per_meter, f64 as si};
use vanetza_sys::vanetza_wrapper::units::CurvatureWrapper as CxxCurvature;

pub type Curvature = si::Curvature;

impl ToCxxUniquePtr<CxxCurvature> for Curvature {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxCurvature> {
        let value = self.get::<radian_per_meter>();
        CxxCurvature::new1(value).within_unique_ptr()
    }
}

impl FromCxxRef<CxxCurvature> for Curvature {
    fn from_cxx_ref(src: &CxxCurvature) -> Self {
        Curvature::new::<radian_per_meter>(src.as_reciprocal_meters())
    }
}
