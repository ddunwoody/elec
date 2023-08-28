/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

use crate::ElecSys;
use elec_sys::libelec_vis_t;

pub struct ElecVis {
    vis: *mut libelec_vis_t,
}

impl ElecVis {
    #[must_use]
    pub fn new(sys: &ElecSys, pos_scale: f64, font_sz: f64) -> ElecVis {
        unsafe {
            ElecVis {
                vis: elec_sys::libelec_vis_new(sys.elec, pos_scale, font_sz),
            }
        }
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
impl Drop for ElecVis {
    fn drop(&mut self) {
        unsafe {
            elec_sys::libelec_vis_destroy(self.vis);
        }
    }
}
