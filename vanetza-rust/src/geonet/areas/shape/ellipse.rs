use super::{CartesianPosition, Shape};
use crate::{
    units::Length,
    utils::{FromCxxRef, ToCxxUniquePtr},
};
use autocxx::prelude::*;
use cxx::UniquePtr;
use vanetza_sys::{
    vanetza::geonet::{geometric_function2, Ellipse as CxxEllipse},
    vanetza_wrapper::geonet::EllipseWrapper as CxxEllipseWrapper,
};

pub struct Ellipse {
    pub a: Length,
    pub b: Length,
}

impl Shape for Ellipse {
    fn geometric_function(&self, position: &CartesianPosition) -> f64 {
        let cxx_ellipse = self.to_cxx_unique_ptr();
        let cxx_position = position.to_cxx_unique_ptr();
        geometric_function2(&cxx_ellipse, &cxx_position)
    }
}

impl FromCxxRef<CxxEllipse> for Ellipse {
    fn from_cxx_ref(ref_: &CxxEllipse) -> Self {
        let wrapper = CxxEllipseWrapper::new(ref_).within_unique_ptr();
        Self::from_cxx_ref(&*wrapper)
    }
}

impl FromCxxRef<CxxEllipseWrapper> for Ellipse {
    fn from_cxx_ref(ref_: &CxxEllipseWrapper) -> Self {
        let a = Length::from_cxx_ref(&ref_.a().within_unique_ptr());
        let b = Length::from_cxx_ref(&ref_.b().within_unique_ptr());
        Self { a, b }
    }
}

impl ToCxxUniquePtr<CxxEllipse> for Ellipse {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxEllipse> {
        let wrapper: UniquePtr<CxxEllipseWrapper> = self.to_cxx_unique_ptr();
        wrapper.inner().within_unique_ptr()
    }
}

impl ToCxxUniquePtr<CxxEllipseWrapper> for Ellipse {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxEllipseWrapper> {
        let Self { a, b } = self;
        let a = a.to_cxx_unique_ptr();
        let b = b.to_cxx_unique_ptr();
        CxxEllipseWrapper::new2(&a, &b).within_unique_ptr()
    }
}
