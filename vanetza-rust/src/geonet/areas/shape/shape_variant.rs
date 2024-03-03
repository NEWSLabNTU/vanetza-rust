use super::{CartesianPosition, Circle, Ellipse, Rectangle, Shape};

pub enum ShapeVariant {
    Circle(Circle),
    Rectangle(Rectangle),
    Ellipse(Ellipse),
}

impl Shape for ShapeVariant {
    fn geometric_function(&self, position: &CartesianPosition) -> f64 {
        match self {
            ShapeVariant::Circle(shape) => shape.geometric_function(position),
            ShapeVariant::Rectangle(shape) => shape.geometric_function(position),
            ShapeVariant::Ellipse(shape) => shape.geometric_function(position),
        }
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
