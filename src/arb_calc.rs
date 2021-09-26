#![allow(non_camel_case_types)]

//! *See the [ARB documentation](https://arblib.org/).

use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpz::fmpz;
use flint_sys::fmpq::fmpq;
use crate::fmpr::fmpr_struct;
use libc::{c_char, c_int, FILE};


pub type arb_calc_func_t = ::std::option::Option<
        out: arb_ptr,
        inp: *mut arb_struct,
        param: *mut c_void,
        order: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> c_int,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arf_interval_struct {
    pub a: arf_struct,
    pub b: arf_struct,
}

pub type arf_interval_t = [arf_interval_struct; 1usize];
pub type arf_interval_ptr = *mut arf_interval_struct;
pub type arf_interval_srcptr = *const arf_interval_struct;

extern "C" {
    pub static mut arb_calc_verbose: c_int;
    pub fn arb_calc_partition(
        L: *mut arf_interval_struct,
        R: *mut arf_interval_struct,
        func: arb_calc_func_t,
        param: *mut c_void,
        block: *mut arf_interval_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_calc_isolate_roots(
        blocks: *mut arf_interval_ptr,
        flags: *mut *mut c_int,
        func: arb_calc_func_t,
        param: *mut c_void,
        block: *mut arf_interval_struct,
        maxdepth: mp_limb_signed_t,
        maxeval: mp_limb_signed_t,
        maxfound: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn arb_calc_refine_root_bisect(
        r: *mut arf_interval_struct,
        func: arb_calc_func_t,
        param: *mut c_void,
        start: *mut arf_interval_struct,
        iter: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_calc_newton_conv_factor(
        conv_factor: *mut arf_struct,
        func: arb_calc_func_t,
        param: *mut c_void,
        conv_region: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_calc_newton_step(
        xnew: *mut arb_struct,
        func: arb_calc_func_t,
        param: *mut c_void,
        x: *mut arb_struct,
        conv_region: *mut arb_struct,
        conv_factor: *mut arf_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_calc_refine_root_newton(
        r: *mut arb_struct,
        func: arb_calc_func_t,
        param: *mut c_void,
        start: *mut arb_struct,
        conv_region: *mut arb_struct,
        conv_factor: *mut arf_struct,
        eval_extra_prec: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> c_int;
}
