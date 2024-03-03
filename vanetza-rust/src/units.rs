use uom::si::f64 as si;

pub type Acceleration = si::Acceleration;
pub type Angle = si::Angle;
pub type AngularVelocity = si::AngularVelocity;
pub type Area = si::Area;
pub type Curvature = si::Curvature;
pub type Frequency = si::Frequency;
pub type Length = si::Length;
pub type Duration = chrono::Duration;
pub type Velocity = si::Velocity;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GeoAngle(pub si::Angle);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TrueNorth(pub si::Angle);
