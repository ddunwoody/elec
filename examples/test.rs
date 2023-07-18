use elec::System;
use elec_sys::{
    comp2info, comp_get_in_amps, comp_get_incap_volts, comp_get_out_amps, comp_get_out_pwr,
    comp_get_out_volts, elec_comp_t, elec_comp_type_t, gen_set_rpm_cb, walk_comps,
};
use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    if args.len() != 2 {
        println!("Must specify a libelec file");
    }

    configure_acfutils("test");
    let sys = System::new(&args[1]);
    unsafe {
        walk_comps(sys.sys(), Some(setup_binds), null_mut());
    }

    if !sys.can_start() {
        println!("Cannot start system");
        return;
    }

    if sys.start() {
        println!("System started");
        for _ in 0..10 {
            unsafe {
                walk_comps(sys.sys(), Some(print_batts), null_mut());
            }
            thread::sleep(Duration::from_millis(100));
            println!("loop");
        }
    }

    sys.stop();
    println!("System stopped");
}

extern "C" fn get_gen_rpm(_comp: *mut elec_comp_t, _userinfo: *mut c_void) -> f64 {
    100.0
}

unsafe extern "C" fn setup_binds(comp: *mut elec_comp_t, _userinfo: *mut c_void) {
    let info = *comp2info(comp);
    match info.type_ {
        elec_comp_type_t::ELEC_GEN => {
            gen_set_rpm_cb(comp, Some(get_gen_rpm));
        }
        _ => {}
    }
}

unsafe extern "C" fn print_batts(comp: *mut elec_comp_t, _userinfo: *mut c_void) {
    let info = *comp2info(comp);

    match info.type_ {
        elec_comp_type_t::ELEC_LOAD => {
            let name = CStr::from_ptr(info.name).to_str().unwrap();
            let out_volts = comp_get_out_volts(comp);
            let out_amps = comp_get_out_amps(comp);
            let out_pwr = comp_get_out_pwr(comp);
            let incap_volts = comp_get_incap_volts(comp);
            let in_amps = comp_get_in_amps(comp);
            println!("{name}, {out_volts}, {out_amps}, {out_pwr}, {incap_volts}, {in_amps}");
        }
        _ => {}
    }
}
fn configure_acfutils(prefix: &str) {
    unsafe {
        acfutils_sys::crc64_init();
        acfutils_sys::crc64_srand(0);
        let log_prefix = CString::new(prefix).unwrap();
        acfutils_sys::log_init(Some(debug_print), log_prefix.as_ptr());
    }
}

extern "C" fn debug_print(msg: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(msg) };
    let str_slice: &str = c_str.to_str().unwrap();
    println!("{str_slice}");
}
