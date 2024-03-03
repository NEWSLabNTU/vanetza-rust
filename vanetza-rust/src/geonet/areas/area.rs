use super::{GeodeticPosition, ShapeVariant};
use crate::{
    units::Angle,
    utils::{FromCxxRef, ToCxxUniquePtr},
};
use autocxx::prelude::*;
use cxx::UniquePtr;
use vanetza_sys::{
    vanetza::geonet::GeodeticPosition as CxxGeodeticPosition,
    vanetza_wrapper::geonet::{
        area_size, inside_or_at_border, GeonetAreaWrapper as CxxAreaWrapper,
    },
};

pub struct Area {
    pub shape: ShapeVariant,
    pub position: GeodeticPosition,
    pub angle: Angle,
}

impl Area {
    pub fn inside_or_at_border(&self, position: &GeodeticPosition) -> bool {
        let cxx_area: UniquePtr<CxxAreaWrapper> = self.to_cxx_unique_ptr();
        let cxx_position: UniquePtr<CxxGeodeticPosition> = position.to_cxx_unique_ptr();
        inside_or_at_border(&cxx_area, &cxx_position)
    }

    pub fn area_size(&self) -> crate::units::Area {
        let cxx_area: UniquePtr<CxxAreaWrapper> = self.to_cxx_unique_ptr();
        let size = area_size(&cxx_area).within_unique_ptr();
        crate::units::Area::from_cxx_ref(&size)
    }
}

impl FromCxxRef<CxxAreaWrapper> for Area {
    fn from_cxx_ref(ref_: &CxxAreaWrapper) -> Self {
        todo!()
    }
}

impl ToCxxUniquePtr<CxxAreaWrapper> for Area {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxAreaWrapper> {
        todo!()
    }
}
