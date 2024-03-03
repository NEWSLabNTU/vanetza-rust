mod circle;
mod ellipse;
mod rectangle;
mod shape_variant;

pub use circle::*;
pub use ellipse::*;
pub use rectangle::*;
pub use shape_variant::*;

use super::CartesianPosition;

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
