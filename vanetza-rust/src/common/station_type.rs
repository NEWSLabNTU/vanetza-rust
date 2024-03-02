use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u16)]
pub enum StationType {
    Unknown = 0,
    Pedestrian = 1,
    Cyclist = 2,
    Moped = 3,
    Motorcycle = 4,
    PassengerCar = 5,
    Bus = 6,
    LightTruck = 7,
    HeavyTruck = 8,
    Trailer = 9,
    SpecialVehicle = 10,
    Tram = 11,
    RSU = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct CStationType(pub(crate) u16);

impl TryFrom<CStationType> for StationType {
    type Error = CStationType;

    fn try_from(src: CStationType) -> Result<Self, Self::Error> {
        Self::from_u16(src.0).ok_or(src)
    }
}

impl From<StationType> for CStationType {
    fn from(src: StationType) -> Self {
        Self(src as u16)
    }
}
