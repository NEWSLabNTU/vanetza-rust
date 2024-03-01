use vanetza_sys as sys;

pub struct Runtime(pub(crate) sys::c_runtime);

impl Runtime {
    pub fn new_manual_runtime() -> Self {
        unsafe { Self(sys::manual_runtime_new()) }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            sys::runtime_del(self.0);
        }
    }
}
