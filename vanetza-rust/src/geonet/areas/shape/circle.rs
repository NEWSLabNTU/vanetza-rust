use super::{CartesianPosition, Shape};
use crate::{
    units::Length,
    utils::{FromCxxRef, ToCxxUniquePtr},
};
use autocxx::prelude::*;
use cxx::UniquePtr;
use vanetza_sys::{
    vanetza::geonet::{geometric_function, Circle as CxxCircle},
    vanetza_wrapper::geonet::CircleWrapper as CxxCircleWrapper,
};

pub struct Circle {
    pub r: Length,
}

impl Shape for Circle {
    fn geometric_function(&self, position: &CartesianPosition) -> f64 {
        let cxx_circle = self.to_cxx_unique_ptr();
        let cxx_position = position.to_cxx_unique_ptr();
        geometric_function(&cxx_circle, &cxx_position)
    }
}

impl FromCxxRef<CxxCircle> for Circle {
    fn from_cxx_ref(ref_: &CxxCircle) -> Self {
        let wrapper = CxxCircleWrapper::new(ref_).within_unique_ptr();
        Self::from_cxx_ref(&*wrapper)
    }
}

impl FromCxxRef<CxxCircleWrapper> for Circle {
    fn from_cxx_ref(ref_: &CxxCircleWrapper) -> Self {
        let r = Length::from_cxx_ref(&ref_.r().within_unique_ptr());
        Self { r }
    }
}

impl ToCxxUniquePtr<CxxCircle> for Circle {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxCircle> {
        let wrapper: UniquePtr<CxxCircleWrapper> = self.to_cxx_unique_ptr();
        wrapper.inner().within_unique_ptr()
    }
}

impl ToCxxUniquePtr<CxxCircleWrapper> for Circle {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxCircleWrapper> {
        let Self { r } = self;
        let r = r.to_cxx_unique_ptr();
        CxxCircleWrapper::new2(&r).within_unique_ptr()
    }
}
