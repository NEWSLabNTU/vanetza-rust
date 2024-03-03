use crate::units::{Angle, GeoAngle, Length};
use vanetza_sys::vanetza::geonet::Area as CxxArea;

pub struct Area {
    pub shape: ShapeVariant,
    pub position: GeodeticPosition,
    pub angle: Angle,
}

impl Area {
    pub fn inside_or_at_border(area: &Area, position: &GeodeticPosition) -> bool {
        todo!();
    }

    pub fn area_size(&self) -> Area {
        todo!();
    }

    // pub fn to_cxx(&self) -> UniquePtr<CxxArea> {
    //     todo!();
    // }

    pub(crate) fn from_cxx(cxx: &CxxArea) -> Self {
        todo!();
    }
}

pub struct GeodeticPosition {
    pub latitude: GeoAngle,
    pub longitude: GeoAngle,
}

impl GeodeticPosition {
    pub fn distance_to(&self, other: &Self) -> Length {
        todo!();
    }

    pub fn local_cartesian_of(&self, position: &Self) -> CartesianPosition {
        todo!();
    }

    // pub fn to_cxx(&self) -> UniquePtr<CxxGeodeticPosition> {
    //     todo!();
    // }

    // pub(crate) fn from_cxx(cxx: &CxxGeodeticPosition) -> Self {
    //     todo!();
    // }
}

pub struct CartesianPosition {
    pub x: Length,
    pub y: Length,
}

impl CartesianPosition {
    pub fn canonicalize(&self, azimuth: Angle) -> Self {
        todo!();
    }

    // pub fn to_cxx(&self) -> UniquePtr<CxxGeodeticPosition> {
    //     todo!();
    // }

    // pub(crate) fn from_cxx(cxx: &CxxGeodeticPosition) -> Self {
    //     todo!();
    // }
}

pub trait Shape {
    fn geometric_function(&self, position: &CartesianPosition) -> f64;

    fn inside_shape(&self, p: &CartesianPosition) -> bool {
        self.geometric_function(p) > 0.0
    }

    fn outside_shape(&self, p: &CartesianPosition) -> bool {
        self.geometric_function(p) < 0.0
    }

    fn at_shape_border(&self, p: &CartesianPosition) -> bool {
        self.geometric_function(p) == 0.0
    }

    fn at_center_point(&self, p: &CartesianPosition) -> bool {
        self.geometric_function(p) == 1.0
    }
}

pub enum ShapeVariant {
    Circle(Circle),
    Rectangle(Rectangle),
    Ellipse(Ellipse),
}

impl Shape for ShapeVariant {
    fn geometric_function(&self, position: &CartesianPosition) -> f64 {
        todo!()
    }
}

impl From<Ellipse> for ShapeVariant {
    fn from(v: Ellipse) -> Self {
        Self::Ellipse(v)
    }
}

impl From<Rectangle> for ShapeVariant {
    fn from(v: Rectangle) -> Self {
        Self::Rectangle(v)
    }
}

impl From<Circle> for ShapeVariant {
    fn from(v: Circle) -> Self {
        Self::Circle(v)
    }
}

pub struct Circle {
    pub r: Length,
}

impl Shape for Circle {
    fn geometric_function(&self, position: &CartesianPosition) -> f64 {
        todo!()
    }
}

pub struct Rectangle {
    pub a: Length,
    pub b: Length,
}

impl Shape for Rectangle {
    fn geometric_function(&self, position: &CartesianPosition) -> f64 {
        todo!()
    }
}

pub struct Ellipse {
    pub a: Length,
    pub b: Length,
}

impl Shape for Ellipse {
    fn geometric_function(&self, position: &CartesianPosition) -> f64 {
        todo!()
    }
}
