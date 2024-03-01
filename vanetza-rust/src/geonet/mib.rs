use vanetza_sys as sys;

pub struct MIB(pub(crate) sys::c_mib);

impl Default for MIB {
    fn default() -> Self {
        unsafe { Self(sys::mib_new()) }
    }
}

impl Drop for MIB {
    fn drop(&mut self) {
        unsafe {
            sys::mib_del(self.0);
        }
    }
}
