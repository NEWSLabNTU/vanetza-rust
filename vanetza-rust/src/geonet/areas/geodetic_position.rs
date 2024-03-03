use super::CartesianPosition;
use crate::{
    units::{GeoAngle, Length},
    utils::{FromCxxRef, ToCxxUniquePtr},
};
use autocxx::prelude::*;
use cxx::UniquePtr;
use vanetza_sys::{
    vanetza::geonet::{local_cartesian, GeodeticPosition as CxxGeodeticPosition},
    vanetza_wrapper::geonet::{distance, GeodeticPositionWrapper as CxxGeodeticPositionWrapper},
};

pub struct GeodeticPosition {
    pub latitude: GeoAngle,
    pub longitude: GeoAngle,
}

impl GeodeticPosition {
    pub fn distance_to(&self, other: &Self) -> Length {
        let cxx_lhs: UniquePtr<CxxGeodeticPosition> = self.to_cxx_unique_ptr();
        let cxx_rhs: UniquePtr<CxxGeodeticPosition> = other.to_cxx_unique_ptr();
        let dist = distance(&cxx_lhs, &cxx_rhs).within_unique_ptr();
        Length::from_cxx_ref(&dist)
    }

    pub fn local_cartesian_at(&self, position: &Self) -> CartesianPosition {
        let cxx_origin: UniquePtr<CxxGeodeticPosition> = self.to_cxx_unique_ptr();
        let cxx_position: UniquePtr<CxxGeodeticPosition> = position.to_cxx_unique_ptr();
        let local_pos = local_cartesian(&cxx_origin, &cxx_position).within_unique_ptr();
        CartesianPosition::from_cxx_ref(&*local_pos)
    }
}

impl FromCxxRef<CxxGeodeticPosition> for GeodeticPosition {
    fn from_cxx_ref(ref_: &CxxGeodeticPosition) -> Self {
        let wrapper = CxxGeodeticPositionWrapper::new(ref_).within_unique_ptr();
        Self::from_cxx_ref(&*wrapper)
    }
}

impl FromCxxRef<CxxGeodeticPositionWrapper> for GeodeticPosition {
    fn from_cxx_ref(ref_: &CxxGeodeticPositionWrapper) -> Self {
        let latitude = GeoAngle::from_cxx_ref(&ref_.latitude().within_unique_ptr());
        let longitude = GeoAngle::from_cxx_ref(&ref_.longitude().within_unique_ptr());
        Self {
            latitude,
            longitude,
        }
    }
}

impl ToCxxUniquePtr<CxxGeodeticPosition> for GeodeticPosition {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxGeodeticPosition> {
        let wrapper: UniquePtr<CxxGeodeticPositionWrapper> = self.to_cxx_unique_ptr();
        wrapper.inner().within_unique_ptr()
    }
}

impl ToCxxUniquePtr<CxxGeodeticPositionWrapper> for GeodeticPosition {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxGeodeticPositionWrapper> {
        let Self {
            latitude,
            longitude,
        } = self;
        CxxGeodeticPositionWrapper::new2(
            &latitude.to_cxx_unique_ptr(),
            &longitude.to_cxx_unique_ptr(),
        )
        .within_unique_ptr()
    }
}
