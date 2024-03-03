use crate::utils::{FromCxxRef, ToCxxUniquePtr};
use autocxx::prelude::*;
use cxx::UniquePtr;
use vanetza_sys::vanetza_wrapper::units::DurationWrapper as CxxDuration;

pub type Duration = chrono::Duration;

impl ToCxxUniquePtr<CxxDuration> for Duration {
    fn to_cxx_unique_ptr(&self) -> UniquePtr<CxxDuration> {
        let micros = self.num_microseconds().unwrap();
        let secs = micros as f64 / 1_000_000.0;
        CxxDuration::new1(secs).within_unique_ptr()
    }
}

impl FromCxxRef<CxxDuration> for Duration {
    fn from_cxx_ref(src: &CxxDuration) -> Self {
        let secs = src.as_seconds();
        let micros = (secs * 1_000_000.0) as i64;
        Duration::microseconds(micros)
    }
}
