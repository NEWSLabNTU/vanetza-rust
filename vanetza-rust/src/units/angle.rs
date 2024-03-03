use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use uom::si::{
    angle::{degree, radian},
    f64 as si,
};
use vanetza_sys::vanetza_wrapper::units::{
    AngleWrapper as CxxAngle, GeoAngleWrapper as CxxGeoAngle, TrueNorthWrapper as CxxTrueNorth,
};

pub type Angle = si::Angle;

impl ToCxxUniquePtr<CxxAngle> for Angle {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxAngle> {
        let rads = self.get::<radian>();
        CxxAngle::new1(rads).within_unique_ptr()
    }
}

impl FromCxxRef<CxxAngle> for Angle {
    fn from_cxx_ref(src: &CxxAngle) -> Self {
        let rads = src.as_radians();
        Angle::new::<radian>(rads)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GeoAngle(pub si::Angle);

impl ToCxxUniquePtr<CxxGeoAngle> for GeoAngle {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxGeoAngle> {
        let degs = self.0.get::<degree>();
        CxxGeoAngle::new1(degs).within_unique_ptr()
    }
}

impl FromCxxRef<CxxGeoAngle> for GeoAngle {
    fn from_cxx_ref(src: &CxxGeoAngle) -> Self {
        let degs = src.as_degrees();
        GeoAngle(Angle::new::<degree>(degs))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TrueNorth(pub si::Angle);

impl ToCxxUniquePtr<CxxTrueNorth> for TrueNorth {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxTrueNorth> {
        let degs = self.0.get::<degree>();
        CxxTrueNorth::new1(degs).within_unique_ptr()
    }
}

impl FromCxxRef<CxxTrueNorth> for TrueNorth {
    fn from_cxx_ref(src: &CxxTrueNorth) -> Self {
        let degs = src.as_degrees();
        TrueNorth(Angle::new::<degree>(degs))
    }
}
