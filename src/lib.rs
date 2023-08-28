/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
/*
 * Copyright 2023 Saso Kiselkov. All rights reserved.
 */
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

#[cfg(feature = "xplane")]
pub mod vis;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;

use elec_sys::{
    elec_comp_t, elec_get_load_cb_t, elec_sys_t, elec_user_cb_t, libelec_add_user_cb,
    libelec_batt_get_chg_rel, libelec_batt_get_temp, libelec_batt_set_chg_rel,
    libelec_batt_set_temp, libelec_cb_get, libelec_cb_get_temp, libelec_cb_set,
    libelec_chgr_get_working, libelec_comp_find, libelec_comp_get_autogen, libelec_comp_get_conn,
    libelec_comp_get_eff, libelec_comp_get_failed, libelec_comp_get_in_amps,
    libelec_comp_get_in_freq, libelec_comp_get_in_pwr, libelec_comp_get_in_volts,
    libelec_comp_get_incap_volts, libelec_comp_get_location, libelec_comp_get_name,
    libelec_comp_get_num_conns, libelec_comp_get_out_amps, libelec_comp_get_out_freq,
    libelec_comp_get_out_pwr, libelec_comp_get_out_volts, libelec_comp_get_shorted,
    libelec_comp_get_srcs, libelec_comp_get_type, libelec_comp_is_AC, libelec_comp_is_powered,
    libelec_comp_set_failed, libelec_comp_set_shorted, libelec_comp_set_userinfo, libelec_destroy,
    libelec_gen_set_random_freq, libelec_gen_set_random_volts, libelec_load_set_load_cb,
    libelec_new, libelec_remove_user_cb, libelec_sys_can_start, libelec_sys_get_time_factor,
    libelec_sys_is_started, libelec_sys_set_time_factor, libelec_sys_start, libelec_sys_stop,
    libelec_tie_get_all, libelec_tie_get_list, libelec_tie_get_num_buses, libelec_tie_set_all,
    libelec_tie_set_list, libelec_walk_comps, ELEC_MAX_SRCS,
};

pub struct ElecSys {
    elec: *mut elec_sys_t,
}

impl ElecSys {
    #[must_use]
    pub fn new(filename: &str) -> Option<ElecSys> {
        let elec = unsafe {
            let c_filename = CString::new(filename).unwrap();
            libelec_new(c_filename.as_ptr())
        };
        if elec.is_null() {
            None
        } else {
            Some(ElecSys { elec })
        }
    }
    pub fn start(&mut self) -> bool {
        unsafe { libelec_sys_start(self.elec) }
    }
    pub fn stop(&mut self) {
        unsafe {
            libelec_sys_stop(self.elec);
        }
    }
    #[must_use]
    pub fn is_started(&self) -> bool {
        unsafe { libelec_sys_is_started(self.elec) }
    }
    #[must_use]
    pub fn can_start(&self) -> bool {
        unsafe { libelec_sys_can_start(self.elec) }
    }
    pub fn sys_set_time_factor(&mut self, time_factor: f64) {
        unsafe {
            libelec_sys_set_time_factor(self.elec, time_factor);
        }
    }
    #[must_use]
    pub fn sys_get_time_factor(&self) -> f64 {
        unsafe { libelec_sys_get_time_factor(self.elec) }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn add_user_cb(&mut self, pre: bool, cb: elec_user_cb_t, userinfo: *mut c_void) {
        unsafe {
            libelec_add_user_cb(self.elec, pre, cb, userinfo);
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn remove_user_cb(&mut self, pre: bool, cb: elec_user_cb_t, userinfo: *mut c_void) {
        unsafe {
            libelec_remove_user_cb(self.elec, pre, cb, userinfo);
        }
    }
    #[must_use]
    pub fn comp_find(&self, name: &str) -> Option<ElecComp> {
        let comp = unsafe {
            let c_name = CString::new(name).unwrap();
            libelec_comp_find(self.elec, c_name.as_ptr())
        };
        if comp.is_null() {
            None
        } else {
            Some(ElecComp { comp })
        }
    }
    extern "C" fn comp_walk_cb(comp: *mut elec_comp_t, userinfo: *mut c_void) {
        unsafe {
            let comps = userinfo.cast::<Vec<ElecComp>>();
            (*comps).push(ElecComp { comp });
        }
    }
    pub fn all_comps(&self) -> Vec<ElecComp> {
        let mut comps: Vec<ElecComp> = vec![];
        unsafe {
            let comps_ptr: *mut Vec<ElecComp> = &mut comps;
            libelec_walk_comps(
                self.elec,
                Some(Self::comp_walk_cb),
                comps_ptr.cast::<std::ffi::c_void>(),
            );
        };
        comps
    }
}

impl Drop for ElecSys {
    fn drop(&mut self) {
        unsafe {
            if libelec_sys_is_started(self.elec) {
                libelec_sys_stop(self.elec);
            }
            libelec_destroy(self.elec);
        }
    }
}

#[derive(Clone, Copy)]
pub struct ElecComp {
    comp: *mut elec_comp_t,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum CompType {
    Batt,
    Gen,
    TRU,
    Inv,
    Load,
    Bus,
    CB,
    Shunt,
    Tie,
    Diode,
    LabelBox,
}

impl ElecComp {
    /*
     * Configuration interrogation
     */
    #[must_use]
    pub fn get_name(&self) -> String {
        unsafe {
            CStr::from_ptr(libelec_comp_get_name(self.comp))
                .to_str()
                .unwrap()
                .to_string()
        }
    }
    fn get_type(&self) -> CompType {
        unsafe { std::mem::transmute(libelec_comp_get_type(self.comp)) }
    }
    #[must_use]
    pub fn get_location(&self) -> String {
        unsafe {
            CStr::from_ptr(libelec_comp_get_location(self.comp))
                .to_str()
                .unwrap()
                .to_string()
        }
    }
    #[must_use]
    pub fn get_autogen(&self) -> bool {
        unsafe { libelec_comp_get_autogen(self.comp) }
    }
    #[must_use]
    pub fn get_num_conns(&self) -> usize {
        unsafe { libelec_comp_get_num_conns(self.comp) }
    }
    #[must_use]
    pub fn get_conn(&self, i: usize) -> ElecComp {
        unsafe {
            assert!(i < Self::get_num_conns(self));
            ElecComp {
                comp: libelec_comp_get_conn(self.comp, i),
            }
        }
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn is_AC(&self) -> bool {
        unsafe { libelec_comp_is_AC(self.comp) }
    }
    /*
     * Electrical state interrogation
     */
    #[must_use]
    pub fn in_volts(&self) -> f64 {
        unsafe { libelec_comp_get_in_volts(self.comp) }
    }
    #[must_use]
    pub fn out_volts(&self) -> f64 {
        unsafe { libelec_comp_get_out_volts(self.comp) }
    }
    #[must_use]
    pub fn in_amps(&self) -> f64 {
        unsafe { libelec_comp_get_in_amps(self.comp) }
    }
    #[must_use]
    pub fn out_amps(&self) -> f64 {
        unsafe { libelec_comp_get_out_amps(self.comp) }
    }
    #[must_use]
    pub fn in_pwr(&self) -> f64 {
        unsafe { libelec_comp_get_in_pwr(self.comp) }
    }
    #[must_use]
    pub fn out_pwr(&self) -> f64 {
        unsafe { libelec_comp_get_out_pwr(self.comp) }
    }
    #[must_use]
    pub fn in_freq(&self) -> f64 {
        unsafe { libelec_comp_get_in_freq(self.comp) }
    }
    #[must_use]
    pub fn out_freq(&self) -> f64 {
        unsafe { libelec_comp_get_out_freq(self.comp) }
    }
    #[must_use]
    pub fn incap_volts(&self) -> f64 {
        unsafe { libelec_comp_get_incap_volts(self.comp) }
    }
    #[must_use]
    pub fn is_powered(&self) -> bool {
        unsafe { libelec_comp_is_powered(self.comp) }
    }
    #[must_use]
    pub fn get_eff(&self) -> f64 {
        unsafe { libelec_comp_get_eff(self.comp) }
    }
    #[must_use]
    pub fn get_srcs(&self) -> Vec<ElecComp> {
        const MAX_SRCS: usize = ELEC_MAX_SRCS as usize;
        let mut srcs_array: [*mut elec_comp_t; MAX_SRCS] = [std::ptr::null_mut(); MAX_SRCS];
        let n = unsafe { libelec_comp_get_srcs(self.comp, srcs_array.as_mut_ptr()) as usize };
        let mut srcs: Vec<ElecComp> = vec![];
        for src in srcs_array.iter().take(n) {
            srcs.push(ElecComp { comp: *src });
        }
        srcs
    }
    /*
     * Failures
     */
    pub fn set_failed(&mut self, failed: bool) {
        unsafe { libelec_comp_set_failed(self.comp, failed) }
    }
    #[must_use]
    pub fn get_failed(&self) -> bool {
        unsafe { libelec_comp_get_failed(self.comp) }
    }
    pub fn set_shorted(&mut self, shorted: bool) {
        unsafe { libelec_comp_set_shorted(self.comp, shorted) }
    }
    #[must_use]
    pub fn get_shorted(&self) -> bool {
        unsafe { libelec_comp_get_shorted(self.comp) }
    }
    pub fn set_random_volts(&mut self, stddev: f64) -> f64 {
        unsafe { libelec_gen_set_random_volts(self.comp, stddev) }
    }
    pub fn set_random_freq(&mut self, stddev: f64) -> f64 {
        unsafe { libelec_gen_set_random_freq(self.comp, stddev) }
    }
    /*
     * CBs
     */
    pub fn cb_set(&mut self, set: bool) {
        assert_eq!(self.get_type(), CompType::CB);
        unsafe { libelec_cb_set(self.comp, set) }
    }
    #[must_use]
    pub fn cb_get(&self) -> bool {
        assert_eq!(self.get_type(), CompType::CB);
        unsafe { libelec_cb_get(self.comp) }
    }
    #[must_use]
    pub fn cb_get_temp(&self) -> f64 {
        assert_eq!(self.get_type(), CompType::CB);
        unsafe { libelec_cb_get_temp(self.comp) }
    }
    /*
     * Ties
     */
    pub fn tie_set_list(&mut self, list: &[ElecComp]) {
        assert_eq!(self.get_type(), CompType::Tie);
        let comps: Vec<*const elec_comp_t> =
            list.iter().map(|c| c.comp as *const elec_comp_t).collect();
        unsafe {
            libelec_tie_set_list(
                self.comp,
                comps.len(),
                comps.as_ptr().cast::<*mut elec_sys::elec_comp_s>(),
            );
        }
    }
    pub fn tie_set_all(&mut self, tied: bool) {
        assert_eq!(self.get_type(), CompType::Tie);
        unsafe { libelec_tie_set_all(self.comp, tied) }
    }
    #[must_use]
    pub fn tie_get_all(&self) -> bool {
        assert_eq!(self.get_type(), CompType::Tie);
        unsafe { libelec_tie_get_all(self.comp) }
    }
    #[must_use]
    pub fn tie_get_list(&self) -> Vec<ElecComp> {
        assert_eq!(self.get_type(), CompType::Tie);
        let n_comps = unsafe { libelec_tie_get_num_buses(self.comp) };
        let mut comps: Vec<*mut elec_comp_t> = vec![std::ptr::null_mut(); n_comps];
        unsafe {
            libelec_tie_get_list(
                self.comp,
                n_comps,
                comps.as_mut_ptr().cast::<*mut elec_sys::elec_comp_s>(),
            );
        };
        comps.into_iter().map(|c| ElecComp { comp: c }).collect()
    }
    #[must_use]
    pub fn tie_get_num_buses(&self) -> usize {
        assert_eq!(self.get_type(), CompType::Tie);
        unsafe { libelec_tie_get_num_buses(self.comp) }
    }
    /*
     * Batteries
     */
    #[must_use]
    pub fn batt_get_chg_rel(&self) -> f64 {
        assert_eq!(self.get_type(), CompType::Batt);
        unsafe { libelec_batt_get_chg_rel(self.comp) }
    }
    pub fn batt_set_chg_rel(&mut self, chg_rel: f64) {
        assert_eq!(self.get_type(), CompType::Batt);
        unsafe { libelec_batt_set_chg_rel(self.comp, chg_rel) }
    }
    #[must_use]
    pub fn batt_get_temp(&self) -> f64 {
        assert_eq!(self.get_type(), CompType::Batt);
        unsafe { libelec_batt_get_temp(self.comp) }
    }
    #[allow(non_snake_case)]
    pub fn batt_set_temp(&mut self, T: f64) {
        assert_eq!(self.get_type(), CompType::Batt);
        unsafe { libelec_batt_set_temp(self.comp, T) }
    }
    /*
     * Chargers
     */
    #[must_use]
    pub fn chgr_get_working(&self) -> bool {
        assert_eq!(self.get_type(), CompType::TRU);
        unsafe { libelec_chgr_get_working(self.comp) }
    }

    /*
     * Callbacks
     */
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn comp_set_userinfo(&mut self, userinfo: *mut c_void) {
        unsafe {
            libelec_comp_set_userinfo(self.comp, userinfo);
        }
    }

    pub fn load_set_load_cb(&mut self, cb: elec_get_load_cb_t) {
        assert_eq!(self.get_type(), CompType::Load);
        unsafe {
            libelec_load_set_load_cb(self.comp, cb);
        }
    }

    pub fn load_remove_load_cb(&mut self) {
        assert_eq!(self.get_type(), CompType::Load);
        unsafe {
            libelec_load_set_load_cb(self.comp, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use acfutils_sys::crc64_init;

    use crate::ElecSys;

    fn get_filename() -> String {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("resources/test/test.net")
            .to_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn load_net() {
        unsafe { crc64_init() };
        ElecSys::new(&get_filename()).expect("Failed to load net");
    }

    #[test]
    fn load_and_run_net() {
        unsafe { crc64_init() };
        let mut sys = ElecSys::new(&get_filename()).expect("Failed to load net");
        sys.start();
        std::thread::sleep(std::time::Duration::new(1, 0));
        for comp in &sys.all_comps() {
            if !comp.get_autogen() {
                println!(
                    concat!(
                        "{} ({:?}) U_in:{:.1}V  U_out:{:.1}V  ",
                        "I_in:{:.1}A  I_out:{:.1}A  ",
                        "W_in:{:.1}W  W_out:{:.1}W"
                    ),
                    comp.get_name(),
                    comp.get_type(),
                    comp.in_volts(),
                    comp.out_volts(),
                    comp.in_amps(),
                    comp.out_amps(),
                    comp.in_pwr(),
                    comp.out_pwr()
                );
            }
        }
    }

    #[test]
    fn list_all_comps() {
        unsafe { crc64_init() };
        let sys = ElecSys::new(&get_filename()).expect("Failed to load net");
        for comp in &sys.all_comps() {
            if !comp.get_autogen() {
                println!(
                    concat!(
                        "{} of type {:?}; location: \"{}\"; ",
                        "U_in:{}V  U_out:{}V  I_in:{}A  ",
                        "I_out:{}A  W_in:{}W  W_out:{}W"
                    ),
                    comp.get_name(),
                    comp.get_type(),
                    comp.get_location(),
                    comp.in_volts(),
                    comp.out_volts(),
                    comp.in_amps(),
                    comp.out_amps(),
                    comp.in_pwr(),
                    comp.out_pwr()
                );
            }
        }
    }
}
