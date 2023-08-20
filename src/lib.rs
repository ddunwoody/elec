/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::ffi::{c_void, CStr, CString};

use elec_sys::{elec_comp_info_t, elec_comp_t, elec_comp_type_t, elec_sys_t};

#[cfg(feature = "xplane")]
use elec_sys::libelec_vis_t;

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

    #[cfg(feature = "xplane")]
    #[must_use]
    pub fn vis(&self, pos_scale: f64, font_sz: f64) -> Vis {
        unsafe { Vis::new(elec_sys::libelec_vis_new(self.sys, pos_scale, font_sz)) }
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

    #[must_use]
    pub fn tied_all(&self) -> bool {
        unsafe { elec_sys::libelec_tie_get_all(self.comp) }
    }

    pub fn set_tied_all(&self, tied: bool) {
        unsafe {
            elec_sys::libelec_tie_set_all(self.comp, tied);
        }
    }

    #[must_use]
    pub fn cb_get(&self) -> bool {
        unsafe { elec_sys::libelec_cb_get(self.comp) }
    }

    pub fn cb_set(&self, closed: bool) {
        unsafe {
            elec_sys::libelec_cb_set(self.comp, closed);
        }
    }
}

#[cfg(feature = "xplane")]
pub struct Vis {
    vis: *mut libelec_vis_t,
}

#[cfg(feature = "xplane")]
impl Vis {
    fn new(vis: *mut libelec_vis_t) -> Self {
        Vis { vis }
    }

    pub fn set_open(&self, open: bool) {
        unsafe {
            if open {
                elec_sys::libelec_vis_open(self.vis);
            } else {
                elec_sys::libelec_vis_close(self.vis);
            }
        }
    }

    #[must_use]
    pub fn open(&self) -> bool {
        unsafe { elec_sys::libelec_vis_is_open(self.vis) }
    }

    pub fn set_offset(&self, x: f64, y: f64) {
        unsafe {
            let offset = elec_sys::vect2_t { x, y };
            elec_sys::libelec_vis_set_offset(self.vis, offset);
        }
    }

    #[must_use]
    pub fn offset(&self) -> (f64, f64) {
        unsafe {
            let offset = elec_sys::libelec_vis_get_offset(self.vis);
            (offset.x, offset.y)
        }
    }
}

#[cfg(feature = "xplane")]
impl Drop for Vis {
    fn drop(&mut self) {
        unsafe {
            elec_sys::libelec_vis_destroy(self.vis);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::System;

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
