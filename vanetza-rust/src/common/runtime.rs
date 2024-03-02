use std::pin::Pin;
use vanetza_sys::vanetza::Runtime as CxxRuntime;

pub trait Runtime {
    fn as_cxx_pin_mut(&self) -> Pin<&mut CxxRuntime>;
}
