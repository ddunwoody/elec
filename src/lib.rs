#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use elec_sys::{elec_comp_info_t, elec_comp_t, elec_comp_type_t, elec_sys_t};
use std::ffi::{c_void, CStr, CString};

pub struct System {
    sys: *mut elec_sys_t,
}

impl System {
    pub fn new<S: AsRef<str>>(filename: S) -> Self {
        let filename =
            CString::new(filename.as_ref()).expect("Could not create CString from filename");
        let sys = unsafe { elec_sys::libelec_new(filename.as_ptr()) };
        System { sys }
    }

    #[must_use]
    pub fn can_start(&self) -> bool {
        unsafe { elec_sys::libelec_sys_can_start(self.sys) }
    }

    #[must_use]
    pub fn start(&self) -> bool {
        unsafe { elec_sys::libelec_sys_start(self.sys) }
    }

    pub fn stop(&self) {
        unsafe { elec_sys::libelec_sys_stop(self.sys) }
    }

    #[must_use]
    pub fn started(&self) -> bool {
        unsafe { elec_sys::libelec_sys_is_started(self.sys) }
    }

    pub fn set_time_factor(&self, factor: f64) {
        unsafe { elec_sys::libelec_sys_set_time_factor(self.sys, factor) }
    }

    #[must_use]
    pub fn time_factor(&self) -> f64 {
        unsafe { elec_sys::libelec_sys_get_time_factor(self.sys) }
    }

    pub fn comp_find<T: Into<Vec<u8>>>(&self, name: T) -> Option<Comp> {
        let name = CString::new(name).expect("Could not create CString from name");
        let comp = unsafe { elec_sys::libelec_comp_find(self.sys, name.as_ptr()) };
        if comp.is_null() {
            None
        } else {
            Some(Comp::new(comp))
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn walk_comps(
        &self,
        cb: Option<unsafe extern "C" fn(*mut elec_comp_t, *mut c_void)>,
        userinfo: *mut c_void,
    ) {
        unsafe { elec_sys::libelec_walk_comps(self.sys, cb, userinfo) }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        if self.started() {
            self.stop();
        }
        unsafe { elec_sys::libelec_destroy(self.sys) }
    }
}

pub struct Comp {
    comp: *mut elec_comp_t,
}

impl Comp {
    fn new(comp: *mut elec_comp_t) -> Self {
        Comp { comp }
    }

    #[must_use]
    pub fn info(&self) -> *const elec_comp_info_t {
        unsafe { elec_sys::libelec_comp2info(self.comp) }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        unsafe {
            let info = *self.info();
            CStr::from_ptr(info.name)
                .to_str()
                .expect("Unable to create str from name")
        }
    }

    #[must_use]
    pub fn type_(&self) -> elec_comp_type_t {
        unsafe { (*self.info()).type_ }
    }

    #[must_use]
    pub fn ac(&self) -> bool {
        unsafe { elec_sys::libelec_comp_is_AC(self.comp) }
    }

    #[must_use]
    pub fn num_conns(&self) -> usize {
        unsafe { elec_sys::libelec_comp_get_num_conns(self.comp) }
    }

    #[must_use]
    pub fn conn(&self, i: usize) -> Comp {
        unsafe {
            let comp = elec_sys::libelec_comp_get_conn(self.comp, i);
            Comp::new(comp)
        }
    }

    #[must_use]
    pub fn in_volts(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_in_volts(self.comp) }
    }

    #[must_use]
    pub fn out_volts(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_out_volts(self.comp) }
    }

    #[must_use]
    pub fn in_amps(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_in_amps(self.comp) }
    }

    #[must_use]
    pub fn out_amps(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_out_amps(self.comp) }
    }

    #[must_use]
    pub fn in_pwr(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_in_pwr(self.comp) }
    }

    #[must_use]
    pub fn out_pwr(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_out_pwr(self.comp) }
    }

    #[must_use]
    pub fn in_freq(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_in_freq(self.comp) }
    }

    #[must_use]
    pub fn out_freq(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_out_freq(self.comp) }
    }

    #[must_use]
    pub fn incap_volts(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_incap_volts(self.comp) }
    }

    #[must_use]
    pub fn powered(&self) -> bool {
        unsafe { elec_sys::libelec_comp_is_powered(self.comp) }
    }

    #[must_use]
    pub fn eff(&self) -> f64 {
        unsafe { elec_sys::libelec_comp_get_eff(self.comp) }
    }

    #[must_use]
    pub fn srcs(&self) -> Vec<Comp> {
        unimplemented!()
    }

    pub fn set_failed(&self, failed: bool) {
        unsafe { elec_sys::libelec_comp_set_failed(self.comp, failed) }
    }

    #[must_use]
    pub fn failed(&self) -> bool {
        unsafe { elec_sys::libelec_comp_get_failed(self.comp) }
    }

    pub fn set_shorted(&self, shorted: bool) {
        unsafe { elec_sys::libelec_comp_set_shorted(self.comp, shorted) }
    }

    #[must_use]
    pub fn shorted(&self) -> bool {
        unsafe { elec_sys::libelec_comp_get_shorted(self.comp) }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn set_userinfo(&self, info: *mut c_void) {
        elec_sys::libelec_comp_set_userinfo(self.comp, info);
    }

    #[must_use]
    pub fn userinfo(&self) -> *mut c_void {
        unsafe { elec_sys::libelec_comp_get_userinfo(self.comp) }
    }
}

#[cfg(test)]
mod tests {
    use crate::System;
    use std::path::Path;

    #[test]
    fn can_load_and_run_system() {
        let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/test.elec");
        unsafe {
            acfutils_sys::crc64_init();
            acfutils_sys::crc64_srand(0);
        }

        let system = System::new(filename.to_str().unwrap());

        assert!(system.can_start());

        let batt = system
            .comp_find("MAIN_BATT")
            .expect("Unable to find battery");

        assert!(system.start());
        assert!(system.started());
        std::thread::sleep(std::time::Duration::from_millis(50));

        assert!((batt.out_volts() - 25.4).abs() < f64::EPSILON);

        system.stop();
        assert!(!system.started());
        drop(system);
    }
}
