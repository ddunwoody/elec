use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use elec_sys::{comp2info, elec_comp_t, elec_comp_type_t, gen_set_rpm_cb};

use elec::System;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Must specify a libelec file");
    }

    configure_acfutils("test");
    let sys = System::new(&args[1]);
    unsafe {
        sys.walk_comps(Some(setup_binds), null_mut());
    }

    if !sys.can_start() {
        println!("Cannot start system");
        return;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if sys.start() {
        println!("System started (press Ctrl-C to stop)");
        while running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(100));
            if let Some(load) = sys.comp_find("LOAD_1") {
                println!(
                    "{}: inputs {:.2}V {:.2}A {:.2}W",
                    load.name(),
                    load.in_volts(),
                    load.in_amps(),
                    load.in_pwr(),
                );
            }
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
    if info.type_ == elec_comp_type_t::ELEC_GEN {
        gen_set_rpm_cb(comp, Some(get_gen_rpm));
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
