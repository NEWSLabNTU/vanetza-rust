use vanetza_sys as sys;

pub trait Runtime {
    fn to_raw_ptr(&self) -> sys::c_runtime;
}
