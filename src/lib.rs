#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use elec_sys::{destroy, elec_sys_t, new, sys_can_start, sys_is_started, sys_start, sys_stop};
use std::ffi::CString;

pub struct System {
    sys: *mut elec_sys_t,
}

impl System {
    pub fn new<S: AsRef<str>>(filename: S) -> Self {
        let filename = CString::new(filename.as_ref()).unwrap();
        let sys = unsafe { new(filename.as_ptr()) };
        System { sys }
    }

    pub fn can_start(&self) -> bool {
        unsafe { sys_can_start(self.sys) }
    }

    pub fn start(&self) -> bool {
        unsafe { sys_start(self.sys) }
    }

    pub fn stop(&self) {
        unsafe { sys_stop(self.sys) }
    }

    pub fn is_started(&self) -> bool {
        unsafe { sys_is_started(self.sys) }
    }

    pub fn sys(&self) -> *mut elec_sys_t {
        self.sys
    }
}

impl Drop for System {
    fn drop(&mut self) {
        if self.is_started() {
            self.stop();
        }
        unsafe { destroy(self.sys) }
    }
}
