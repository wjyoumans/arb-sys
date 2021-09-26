#![allow(non_camel_case_types)]

//! *See the [ARB documentation](https://arblib.org/).

use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpz::fmpz;
use flint_sys::fmpq::fmpq;
use crate::fmpr::fmpr_struct;
use libc::{c_char, c_int, FILE};


extern "C" {
    pub fn partitions_rademacher_bound(b: *mut arf_struct, n: *mut fmpz, N: mp_limb_t);
}
extern "C" {
    pub fn partitions_hrr_sum_arb(
        x: *mut arb_struct,
        n: *mut fmpz,
        N0: mp_limb_signed_t,
        N: mp_limb_signed_t,
        use_doubles: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn partitions_fmpz_fmpz(p: *mut fmpz, n: *mut fmpz, use_doubles: ::std::os::raw::c_int);
}
extern "C" {
    pub fn partitions_fmpz_ui(p: *mut fmpz, n: mp_limb_t);
}
extern "C" {
    pub fn partitions_fmpz_ui_using_doubles(p: *mut fmpz, n: mp_limb_t);
}
extern "C" {
    pub fn partitions_leading_fmpz(res: *mut arb_struct, n: *mut fmpz, prec: mp_limb_signed_t);
}
