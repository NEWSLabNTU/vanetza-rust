use crate::{
    units::{Angle, Length},
    utils::{FromCxxRef, ToCxxUniquePtr},
};
use autocxx::prelude::*;
use cxx::UniquePtr;
use vanetza_sys::{
    vanetza::geonet::CartesianPosition as CxxCartesianPosition,
    vanetza_wrapper::geonet::{
        canonicalize, CartesianPositionWrapper as CxxCartesianPositionWrapper,
    },
};

pub struct CartesianPosition {
    pub x: Length,
    pub y: Length,
}

impl CartesianPosition {
    pub fn canonicalize(&self, azimuth: Angle) -> Self {
        let cxx_pos: UniquePtr<CxxCartesianPosition> = self.to_cxx_unique_ptr();
        let cxx_angle = azimuth.to_cxx_unique_ptr();
        let output = canonicalize(&cxx_pos, cxx_angle).within_unique_ptr();
        Self::from_cxx_ref(&*output)
    }
}

impl FromCxxRef<CxxCartesianPosition> for CartesianPosition {
    fn from_cxx_ref(ref_: &CxxCartesianPosition) -> Self {
        let wrapper = CxxCartesianPositionWrapper::new(ref_).within_unique_ptr();
        Self::from_cxx_ref(&*wrapper)
    }
}

impl FromCxxRef<CxxCartesianPositionWrapper> for CartesianPosition {
    fn from_cxx_ref(ref_: &CxxCartesianPositionWrapper) -> Self {
        let x = Length::from_cxx_ref(&ref_.x().within_unique_ptr());
        let y = Length::from_cxx_ref(&ref_.y().within_unique_ptr());
        Self { x, y }
    }
}

impl ToCxxUniquePtr<CxxCartesianPosition> for CartesianPosition {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxCartesianPosition> {
        let wrapper: UniquePtr<CxxCartesianPositionWrapper> = self.to_cxx_unique_ptr();
        wrapper.inner().within_unique_ptr()
    }
}

impl ToCxxUniquePtr<CxxCartesianPositionWrapper> for CartesianPosition {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxCartesianPositionWrapper> {
        let Self { x, y } = self;
        CxxCartesianPositionWrapper::new2(&x.to_cxx_unique_ptr(), &y.to_cxx_unique_ptr())
            .within_unique_ptr()
    }
}
