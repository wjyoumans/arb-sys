#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpz::fmpz;
use flint_sys::fmpq::fmpq;
use crate::fmpr::fmpr_struct;
use libc::{c_char, c_int, FILE};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mag_struct {
    pub exp: fmpz,
    pub man: mp_limb_t,
}

pub type mag_t = [mag_struct; 1usize];
pub type mag_ptr = *mut mag_struct;
pub type mag_srcptr = *const mag_struct;

extern "C" {
    pub fn mag_init(x: *mut mag_struct);
    pub fn mag_init_set(x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_clear(x: *mut mag_struct);
    pub fn mag_swap(x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_set(x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_zero(x: *mut mag_struct);
    pub fn mag_one(x: *mut mag_struct);
    pub fn mag_is_special(x: *mut mag_struct) -> c_int;
    pub fn mag_is_zero(x: *mut mag_struct) -> c_int;
    pub fn mag_inf(x: *mut mag_struct);
    pub fn mag_is_inf(x: *mut mag_struct) -> c_int;
    pub fn mag_is_finite(x: *mut mag_struct) -> c_int;
    pub fn mag_equal(x: *mut mag_struct, y: *mut mag_struct) -> c_int;
    pub fn mag_mul(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_mul_lower(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_addmul(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_add_2exp_fmpz(z: *mut mag_struct, x: *mut mag_struct, e: *mut fmpz);
    pub fn mag_add(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_add_lower(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_add_ui(z: *mut mag_struct, x: *mut mag_struct, y: mp_limb_t);
    pub fn mag_add_ui_lower(res: *mut mag_struct, x: *mut mag_struct, y: mp_limb_t);
    pub fn mag_add_ui_2exp_si(
        z: *mut mag_struct,
        x: *mut mag_struct,
        y: mp_limb_t,
        e: mp_limb_signed_t,
    );
    pub fn mag_div(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_div_lower(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_inv(res: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_inv_lower(res: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_mul_2exp_si(z: *mut mag_struct, x: *mut mag_struct, y: mp_limb_signed_t);
    pub fn mag_mul_2exp_fmpz(z: *mut mag_struct, x: *mut mag_struct, y: *mut fmpz);
    pub fn mag_sub(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_sub_lower(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_fast_init_set(x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_fast_zero(x: *mut mag_struct);
    pub fn mag_fast_is_zero(x: *mut mag_struct) -> c_int;
    pub fn mag_fast_mul(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_fast_mul_2exp_si(z: *mut mag_struct, x: *mut mag_struct, y: mp_limb_signed_t);
    pub fn mag_fast_addmul(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_fast_add_2exp_si(z: *mut mag_struct, x: *mut mag_struct, e: mp_limb_signed_t);
    pub fn mag_set_d(z: *mut mag_struct, x: f64);
    pub fn mag_set_d_lower(z: *mut mag_struct, x: f64);
    pub fn mag_set_d_2exp_fmpz(z: *mut mag_struct, c: f64, exp: *mut fmpz);
    pub fn mag_set_d_2exp_fmpz_lower(z: *mut mag_struct, c: f64, exp: *mut fmpz);
    pub fn mag_set_fmpz_2exp_fmpz(z: *mut mag_struct, man: *mut fmpz, exp: *mut fmpz);
    pub fn mag_set_fmpr(x: *mut mag_struct, y: *mut fmpr_struct);
    pub fn mag_get_fmpr(x: *mut fmpr_struct, r: *mut mag_struct);
    pub fn mag_randtest_special(
        x: *mut mag_struct,
        state: *mut flint_rand_s,
        expbits: mp_limb_signed_t,
    );
    pub fn mag_randtest(x: *mut mag_struct, state: *mut flint_rand_s, expbits: mp_limb_signed_t);
    pub fn mag_fprint(file: *mut FILE, x: *mut mag_struct);
    pub fn mag_fprintd(file: *mut FILE, x: *mut mag_struct, d: mp_limb_signed_t);
    pub fn mag_print(x: *mut mag_struct);
    pub fn mag_printd(x: *mut mag_struct, d: mp_limb_signed_t);
    pub fn mag_get_fmpq(y: *mut fmpq, x: *mut mag_struct);
    pub fn mag_get_fmpz(res: *mut fmpz, x: *mut mag_struct);
    pub fn mag_get_fmpz_lower(res: *mut fmpz, x: *mut mag_struct);
    pub fn mag_cmp(x: *mut mag_struct, y: *mut mag_struct) -> c_int;
    pub fn mag_cmp_2exp_si(x: *mut mag_struct, e: mp_limb_signed_t) -> c_int;
    pub fn mag_min(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_max(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn _mag_vec_init(n: mp_limb_signed_t) -> mag_ptr;
    pub fn _mag_vec_clear(v: mag_ptr, n: mp_limb_signed_t);
    pub fn mag_get_d(z: *mut mag_struct) -> f64;
    pub fn mag_get_d_log2_approx(x: *mut mag_struct) -> f64;
    pub fn mag_d_log_upper_bound(x: f64) -> f64;
    pub fn mag_d_log_lower_bound(x: f64) -> f64;
    pub fn mag_log1p(z: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_log_ui(t: *mut mag_struct, n: mp_limb_t);
    pub fn mag_log(z: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_log_lower(z: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_neg_log(z: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_neg_log_lower(z: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_exp(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_exp_lower(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_expinv(res: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_expinv_lower(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_expm1(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_exp_tail(z: *mut mag_struct, x: *mut mag_struct, N: mp_limb_t);
    pub fn mag_sinh(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_sinh_lower(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_cosh(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_cosh_lower(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_pow_ui(z: *mut mag_struct, x: *mut mag_struct, e: mp_limb_t);
    pub fn mag_pow_ui_lower(z: *mut mag_struct, x: *mut mag_struct, e: mp_limb_t);
    pub fn mag_pow_fmpz(z: *mut mag_struct, x: *mut mag_struct, e: *mut fmpz);
    pub fn mag_pow_fmpz_lower(z: *mut mag_struct, x: *mut mag_struct, e: *mut fmpz);
    pub fn mag_const_pi(res: *mut mag_struct);
    pub fn mag_const_pi_lower(res: *mut mag_struct);
    pub fn mag_atan(res: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_atan_lower(res: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_fac_ui(z: *mut mag_struct, n: mp_limb_t);
    pub fn mag_rfac_ui(z: *mut mag_struct, n: mp_limb_t);
    pub fn mag_bin_uiui(res: *mut mag_struct, n: mp_limb_t, k: mp_limb_t);
    pub fn mag_bernoulli_div_fac_ui(z: *mut mag_struct, n: mp_limb_t);
    pub fn mag_set_fmpz_2exp_fmpz_lower(z: *mut mag_struct, man: *mut fmpz, exp: *mut fmpz);
    pub fn mag_sqrt(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_sqrt_lower(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_rsqrt(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_rsqrt_lower(y: *mut mag_struct, x: *mut mag_struct);
    pub fn mag_root(y: *mut mag_struct, x: *mut mag_struct, n: mp_limb_t);
    pub fn mag_hypot(z: *mut mag_struct, x: *mut mag_struct, y: *mut mag_struct);
    pub fn mag_binpow_uiui(b: *mut mag_struct, m: mp_limb_t, n: mp_limb_t);
    pub fn mag_polylog_tail(
        u: *mut mag_struct,
        z: *mut mag_struct,
        sigma: mp_limb_signed_t,
        d: mp_limb_t,
        N: mp_limb_t,
    );
    pub fn mag_geom_series(res: *mut mag_struct, x: *mut mag_struct, n: mp_limb_t);
    pub fn mag_hurwitz_zeta_uiui(res: *mut mag_struct, s: mp_limb_t, a: mp_limb_t);
    pub fn mag_set_ui(z: *mut mag_struct, x: mp_limb_t);
    pub fn mag_set_ui_lower(z: *mut mag_struct, x: mp_limb_t);
    pub fn mag_set_ui_2exp_si(z: *mut mag_struct, v: mp_limb_t, e: mp_limb_signed_t);
    pub fn mag_set_fmpz(z: *mut mag_struct, x: *mut fmpz);
    pub fn mag_set_fmpz_lower(z: *mut mag_struct, x: *mut fmpz);
    pub fn mag_mul_ui(z: *mut mag_struct, x: *mut mag_struct, y: mp_limb_t);
    pub fn mag_mul_ui_lower(z: *mut mag_struct, x: *mut mag_struct, y: mp_limb_t);
    pub fn mag_mul_fmpz(z: *mut mag_struct, x: *mut mag_struct, y: *mut fmpz);
    pub fn mag_mul_fmpz_lower(z: *mut mag_struct, x: *mut mag_struct, y: *mut fmpz);
    pub fn mag_div_ui(z: *mut mag_struct, x: *mut mag_struct, y: mp_limb_t);
    pub fn mag_div_fmpz(z: *mut mag_struct, x: *mut mag_struct, y: *mut fmpz);
    pub fn mag_allocated_bytes(x: *mut mag_struct) -> mp_limb_signed_t;
    pub fn mag_load_str(
        res: *mut mag_struct,
        data: *const c_char,
    ) -> c_int;
    pub fn mag_dump_str(x: *mut mag_struct) -> *mut c_char;
    pub fn mag_load_file(res: *mut mag_struct, stream: *mut FILE) -> c_int;
    pub fn mag_dump_file(stream: *mut FILE, x: *mut mag_struct) -> c_int;
}
