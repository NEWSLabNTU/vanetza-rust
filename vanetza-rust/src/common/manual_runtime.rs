use vanetza_sys as sys;

use super::runtime::Runtime;

pub struct ManualRuntime(pub(crate) sys::c_manual_runtime);

impl Runtime for ManualRuntime {
    fn to_raw_ptr(&self) -> sys::c_runtime {
        // Since ManualRuntime inherits Runtime, the implicit cast
        // should work.
        self.0
    }
}

impl Default for ManualRuntime {
    fn default() -> Self {
        unsafe { Self(sys::manual_runtime_new()) }
    }
}

impl Drop for ManualRuntime {
    fn drop(&mut self) {
        unsafe {
            sys::runtime_del(self.0);
        }
    }
}
