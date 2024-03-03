use super::{CartesianPosition, Shape};
use crate::{
    units::Length,
    utils::{FromCxxRef, ToCxxUniquePtr},
};
use autocxx::prelude::*;
use cxx::UniquePtr;
use vanetza_sys::{
    vanetza::geonet::{geometric_function1, Rectangle as CxxRectangle},
    vanetza_wrapper::geonet::RectangleWrapper as CxxRectangleWrapper,
};

pub struct Rectangle {
    pub a: Length,
    pub b: Length,
}

impl Shape for Rectangle {
    fn geometric_function(&self, position: &CartesianPosition) -> f64 {
        let cxx_rect = self.to_cxx_unique_ptr();
        let cxx_position = position.to_cxx_unique_ptr();
        geometric_function1(&cxx_rect, &cxx_position)
    }
}

impl FromCxxRef<CxxRectangle> for Rectangle {
    fn from_cxx_ref(ref_: &CxxRectangle) -> Self {
        let wrapper = CxxRectangleWrapper::new(ref_).within_unique_ptr();
        Self::from_cxx_ref(&*wrapper)
    }
}

impl FromCxxRef<CxxRectangleWrapper> for Rectangle {
    fn from_cxx_ref(ref_: &CxxRectangleWrapper) -> Self {
        let a = Length::from_cxx_ref(&ref_.a().within_unique_ptr());
        let b = Length::from_cxx_ref(&ref_.b().within_unique_ptr());
        Self { a, b }
    }
}

impl ToCxxUniquePtr<CxxRectangle> for Rectangle {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxRectangle> {
        let wrapper: UniquePtr<CxxRectangleWrapper> = self.to_cxx_unique_ptr();
        wrapper.inner().within_unique_ptr()
    }
}

impl ToCxxUniquePtr<CxxRectangleWrapper> for Rectangle {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxRectangleWrapper> {
        let Self { a, b } = self;
        let a = a.to_cxx_unique_ptr();
        let b = b.to_cxx_unique_ptr();
        CxxRectangleWrapper::new2(&a, &b).within_unique_ptr()
    }
}
