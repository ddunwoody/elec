#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use elec_sys::{elec_comp_info_t, elec_comp_t, elec_sys_t};
use std::ffi::{c_void, CStr, CString};

pub struct System {
    sys: *mut elec_sys_t,
}

impl System {
    pub fn new<S: AsRef<str>>(filename: S) -> Self {
        let filename = CString::new(filename.as_ref()).unwrap();
        let sys = unsafe { elec_sys::new(filename.as_ptr()) };
        System { sys }
    }

    pub fn can_start(&self) -> bool {
        unsafe { elec_sys::sys_can_start(self.sys) }
    }

    pub fn start(&self) -> bool {
        unsafe { elec_sys::sys_start(self.sys) }
    }

    pub fn stop(&self) {
        unsafe { elec_sys::sys_stop(self.sys) }
    }

    pub fn started(&self) -> bool {
        unsafe { elec_sys::sys_is_started(self.sys) }
    }

    pub fn set_time_factor(&self, factor: f64) {
        unsafe { elec_sys::sys_set_time_factor(self.sys, factor) }
    }

    pub fn time_factor(&self) -> f64 {
        unsafe { elec_sys::sys_get_time_factor(self.sys) }
    }

    pub fn comp_find<T: Into<Vec<u8>>>(&self, name: T) -> Option<Comp> {
        let name = CString::new(name).unwrap();
        let comp = unsafe { elec_sys::comp_find(self.sys, name.as_ptr()) };
        if comp.is_null() {
            None
        } else {
            Some(Comp::new(comp))
        }
    }

    pub fn sys(&self) -> *mut elec_sys_t {
        self.sys
    }
}

impl Drop for System {
    fn drop(&mut self) {
        if self.started() {
            self.stop();
        }
        unsafe { elec_sys::destroy(self.sys) }
    }
}

pub struct Comp {
    comp: *mut elec_comp_t,
}

impl Comp {
    fn new(comp: *mut elec_comp_t) -> Self {
        Comp { comp }
    }

    pub fn info(&self) -> *const elec_comp_info_t {
        unsafe { elec_sys::comp2info(self.comp) }
    }

    pub fn name(&self) -> &str {
        unsafe {
            let info = *self.info();
            CStr::from_ptr(info.name)
                .to_str()
                .expect("Unable to create str from name")
        }
    }

    pub fn ac(&self) -> bool {
        unsafe { elec_sys::comp_is_AC(self.comp) }
    }

    pub fn num_conns(&self) -> usize {
        unsafe { elec_sys::comp_get_num_conns(self.comp) }
    }

    pub fn in_volts(&self) -> f64 {
        unsafe { elec_sys::comp_get_in_volts(self.comp) }
    }

    pub fn out_volts(&self) -> f64 {
        unsafe { elec_sys::comp_get_out_volts(self.comp) }
    }

    pub fn in_amps(&self) -> f64 {
        unsafe { elec_sys::comp_get_in_amps(self.comp) }
    }

    pub fn out_amps(&self) -> f64 {
        unsafe { elec_sys::comp_get_out_amps(self.comp) }
    }

    pub fn in_pwr(&self) -> f64 {
        unsafe { elec_sys::comp_get_in_pwr(self.comp) }
    }

    pub fn out_pwr(&self) -> f64 {
        unsafe { elec_sys::comp_get_out_pwr(self.comp) }
    }

    pub fn in_freq(&self) -> f64 {
        unsafe { elec_sys::comp_get_in_freq(self.comp) }
    }

    pub fn out_freq(&self) -> f64 {
        unsafe { elec_sys::comp_get_out_freq(self.comp) }
    }

    pub fn incap_volts(&self) -> f64 {
        unsafe { elec_sys::comp_get_incap_volts(self.comp) }
    }

    pub fn powered(&self) -> bool {
        unsafe { elec_sys::comp_is_powered(self.comp) }
    }

    pub fn set_failed(&self, failed: bool) {
        unsafe { elec_sys::comp_set_failed(self.comp, failed) }
    }

    pub fn failed(&self) -> bool {
        unsafe { elec_sys::comp_get_failed(self.comp) }
    }

    pub fn set_shorted(&self, shorted: bool) {
        unsafe { elec_sys::comp_set_shorted(self.comp, shorted) }
    }

    pub fn shorted(&self) -> bool {
        unsafe { elec_sys::comp_get_shorted(self.comp) }
    }

    pub fn set_userinfo(&self, info: *mut c_void) {
        unsafe { elec_sys::comp_set_userinfo(self.comp, info) }
    }

    pub fn userinfo(&self) -> *mut c_void {
        unsafe { elec_sys::comp_get_userinfo(self.comp) }
    }
}
