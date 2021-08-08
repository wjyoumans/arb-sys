#![allow(non_camel_case_types)]

//! *See the [ARB documentation](https://arblib.org/).

use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpz::fmpz;
use flint_sys::fmpq::fmpq;

use crate::fmpr::fmpr_struct;
use crate::mag::mag_struct;
use libc::{c_char, c_int, FILE};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mantissa_noptr_struct {
    pub d: [mp_limb_t; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mantissa_ptr_struct {
    pub alloc: mp_size_t,
    pub d: mp_ptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mantissa_struct {
    pub noptr: mantissa_noptr_struct,
    pub ptr: mantissa_ptr_struct,
}


#[repr(C)]
#[derive(Clone)]
pub struct arf_struct {
    pub exp: fmpz,
    pub size: mp_size_t,
    pub d: mantissa_struct,
}

pub type arf_t = [arf_struct; 1usize];
pub type arf_ptr = *mut arf_struct;
pub type arf_srcptr = *const arf_struct;

extern "C" {
    pub fn arf_rounds_down(
        rnd: c_int,
        sgnbit: c_int,
    ) -> c_int;
    pub fn arf_rounds_up(
        rnd: c_int,
        sgnbit: c_int,
    ) -> c_int;
    pub fn arf_rnd_to_mpfr(rnd: c_int) -> mpfr_rnd_t;
    pub fn _arf_promote(x: *mut arf_struct, n: mp_size_t);
    pub fn _arf_demote(x: *mut arf_struct);
    pub fn arf_init(x: *mut arf_struct);
    pub fn arf_clear(x: *mut arf_struct);
    pub fn arf_zero(x: *mut arf_struct);
    pub fn arf_pos_inf(x: *mut arf_struct);
    pub fn arf_neg_inf(x: *mut arf_struct);
    pub fn arf_nan(x: *mut arf_struct);
    pub fn arf_is_special(x: *mut arf_struct) -> c_int;
    pub fn arf_is_zero(x: *mut arf_struct) -> c_int;
    pub fn arf_is_pos_inf(x: *mut arf_struct) -> c_int;
    pub fn arf_is_neg_inf(x: *mut arf_struct) -> c_int;
    pub fn arf_is_nan(x: *mut arf_struct) -> c_int;
    pub fn arf_is_normal(x: *mut arf_struct) -> c_int;
    pub fn arf_is_finite(x: *mut arf_struct) -> c_int;
    pub fn arf_is_inf(x: *mut arf_struct) -> c_int;
    pub fn arf_one(x: *mut arf_struct);
    pub fn arf_is_one(x: *mut arf_struct) -> c_int;
    pub fn arf_sgn(x: *mut arf_struct) -> c_int;
    pub fn arf_cmp(x: *mut arf_struct, y: *mut arf_struct) -> c_int;
    pub fn arf_cmpabs(x: *mut arf_struct, y: *mut arf_struct) -> c_int;
    pub fn arf_cmpabs_ui(x: *mut arf_struct, y: mp_limb_t) -> c_int;
    pub fn arf_cmpabs_d(x: *mut arf_struct, y: f64) -> c_int;
    pub fn arf_cmp_si(x: *mut arf_struct, y: mp_limb_signed_t) -> c_int;
    pub fn arf_cmp_ui(x: *mut arf_struct, y: mp_limb_t) -> c_int;
    pub fn arf_cmp_d(x: *mut arf_struct, y: f64) -> c_int;
    pub fn arf_swap(y: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_set(y: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_neg(y: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_init_set_ui(x: *mut arf_struct, v: mp_limb_t);
    pub fn arf_init_set_si(x: *mut arf_struct, v: mp_limb_signed_t);
    pub fn arf_set_ui(x: *mut arf_struct, v: mp_limb_t);
    pub fn arf_set_si(x: *mut arf_struct, v: mp_limb_signed_t);
    pub fn arf_init_set_shallow(z: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_init_neg_shallow(z: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_init_set_mag_shallow(y: *mut arf_struct, x: *mut mag_struct);
    pub fn arf_init_neg_mag_shallow(z: *mut arf_struct, x: *mut mag_struct);
    pub fn arf_cmpabs_mag(x: *mut arf_struct, y: *mut mag_struct) -> c_int;
    pub fn arf_mag_cmpabs(x: *mut mag_struct, y: *mut arf_struct) -> c_int;
    pub fn arf_set_mpn(
        y: *mut arf_struct,
        x: mp_srcptr,
        xn: mp_size_t,
        sgnbit: c_int,
    );
    pub fn arf_set_mpz(y: *mut arf_struct, x: *mut __mpz_struct);
    pub fn arf_set_fmpz(y: *mut arf_struct, x: *mut fmpz);
    pub fn _arf_set_round_ui(
        x: *mut arf_struct,
        v: mp_limb_t,
        sgnbit: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn _arf_set_round_uiui(
        z: *mut arf_struct,
        fix: *mut mp_limb_signed_t,
        hi: mp_limb_t,
        lo: mp_limb_t,
        sgnbit: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn _arf_set_round_mpn(
        y: *mut arf_struct,
        exp_shift: *mut mp_limb_signed_t,
        x: mp_srcptr,
        xn: mp_size_t,
        sgnbit: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_set_round_ui(
        x: *mut arf_struct,
        v: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_set_round_si(
        x: *mut arf_struct,
        v: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_set_round_mpz(
        y: *mut arf_struct,
        x: *mut __mpz_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_set_round_fmpz(
        y: *mut arf_struct,
        x: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_set_round(
        y: *mut arf_struct,
        x: *mut arf_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_neg_round(
        y: *mut arf_struct,
        x: *mut arf_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_get_fmpr(y: *mut fmpr_struct, x: *mut arf_struct);
    pub fn arf_set_fmpr(y: *mut arf_struct, x: *mut fmpr_struct);
    pub fn arf_get_mpfr(
        x: *mut __mpfr_struct,
        y: *mut arf_struct,
        rnd: mpfr_rnd_t,
    ) -> c_int;
    pub fn arf_set_mpfr(x: *mut arf_struct, y: *mut __mpfr_struct);
    pub fn arf_equal(x: *mut arf_struct, y: *mut arf_struct) -> c_int;
    pub fn arf_equal_si(x: *mut arf_struct, y: mp_limb_signed_t) -> c_int;
    pub fn arf_min(z: *mut arf_struct, a: *mut arf_struct, b: *mut arf_struct);
    pub fn arf_max(z: *mut arf_struct, a: *mut arf_struct, b: *mut arf_struct);
    pub fn arf_abs(y: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_bits(x: *mut arf_struct) -> mp_limb_signed_t;
    pub fn arf_bot(e: *mut fmpz, x: *mut arf_struct);
    pub fn arf_is_int(x: *mut arf_struct) -> c_int;
    pub fn arf_is_int_2exp_si(x: *mut arf_struct, e: mp_limb_signed_t) -> c_int;
    pub fn arf_cmp_2exp_si(x: *mut arf_struct, e: mp_limb_signed_t) -> c_int;
    pub fn arf_cmpabs_2exp_si(x: *mut arf_struct, e: mp_limb_signed_t) -> c_int;
    pub fn arf_set_si_2exp_si(x: *mut arf_struct, man: mp_limb_signed_t, exp: mp_limb_signed_t);
    pub fn arf_set_ui_2exp_si(x: *mut arf_struct, man: mp_limb_t, exp: mp_limb_signed_t);
    pub fn arf_mul_2exp_si(y: *mut arf_struct, x: *mut arf_struct, e: mp_limb_signed_t);
    pub fn arf_mul_2exp_fmpz(y: *mut arf_struct, x: *mut arf_struct, e: *mut fmpz);
    pub fn arf_set_round_fmpz_2exp(
        y: *mut arf_struct,
        x: *mut fmpz,
        exp: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_abs_bound_lt_2exp_fmpz(b: *mut fmpz, x: *mut arf_struct);
    pub fn arf_abs_bound_le_2exp_fmpz(b: *mut fmpz, x: *mut arf_struct);
    pub fn arf_abs_bound_lt_2exp_si(x: *mut arf_struct) -> mp_limb_signed_t;
    pub fn arf_frexp(man: *mut arf_struct, exp: *mut fmpz, x: *mut arf_struct);
    pub fn arf_get_fmpz_2exp(man: *mut fmpz, exp: *mut fmpz, x: *mut arf_struct);
    pub fn _arf_get_integer_mpn(
        y: mp_ptr,
        x: mp_srcptr,
        xn: mp_size_t,
        exp: mp_limb_signed_t,
    ) -> c_int;
    pub fn _arf_set_mpn_fixed(
        z: *mut arf_struct,
        xp: mp_srcptr,
        xn: mp_size_t,
        fixn: mp_size_t,
        negative: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_get_fmpz(
        z: *mut fmpz,
        x: *mut arf_struct,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_get_si(x: *mut arf_struct, rnd: c_int) -> mp_limb_signed_t;
    pub fn arf_get_fmpz_fixed_fmpz(
        y: *mut fmpz,
        x: *mut arf_struct,
        e: *mut fmpz,
    ) -> c_int;
    pub fn arf_get_fmpz_fixed_si(
        y: *mut fmpz,
        x: *mut arf_struct,
        e: mp_limb_signed_t,
    ) -> c_int;
    pub fn arf_set_fmpz_2exp(x: *mut arf_struct, man: *mut fmpz, exp: *mut fmpz);
    pub fn arf_floor(z: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_ceil(z: *mut arf_struct, x: *mut arf_struct);
    pub fn arf_debug(x: *mut arf_struct);
    pub fn arf_fprint(file: *mut FILE, x: *mut arf_struct);
    pub fn arf_fprintd(file: *mut FILE, y: *mut arf_struct, d: mp_limb_signed_t);
    pub fn arf_print(x: *mut arf_struct);
    pub fn arf_printd(y: *mut arf_struct, d: mp_limb_signed_t);
    pub fn arf_randtest(
        x: *mut arf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arf_randtest_not_zero(
        x: *mut arf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arf_randtest_special(
        x: *mut arf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arf_urandom(
        x: *mut arf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        rnd: c_int,
    );
    pub static mut __arf_mul_tmp: mp_ptr;
    pub static mut __arf_mul_alloc: mp_limb_signed_t;
    pub fn _arf_mul_tmp_cleanup();
    pub fn arf_mul_special(z: *mut arf_struct, x: *mut arf_struct, y: *mut arf_struct);
    pub fn arf_mul_via_mpfr(
        z: *mut arf_struct,
        x: *mut arf_struct,
        y: *mut arf_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_mul_rnd_any(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_mul_rnd_down(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arf_neg_mul(
        z: *mut arf_struct,
        x: *mut arf_struct,
        y: *mut arf_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_mul_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_mul_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_mul_mpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut __mpz_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_mul_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub static mut __arf_add_tmp: mp_ptr;
    pub static mut __arf_add_alloc: mp_limb_signed_t;
    pub fn _arf_add_tmp_cleanup();
    pub fn _arf_add_mpn(
        z: *mut arf_struct,
        xp: mp_srcptr,
        xn: mp_size_t,
        xsgnbit: c_int,
        xexp: *mut fmpz,
        yp: mp_srcptr,
        yn: mp_size_t,
        ysgnbit: c_int,
        shift: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_add(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_add_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_add_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_add_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_add_fmpz_2exp(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut fmpz,
        exp: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sub(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sub_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sub_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sub_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_addmul(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_addmul_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_addmul_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_addmul_mpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut __mpz_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_addmul_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_submul(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_submul_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_submul_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_submul_mpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut __mpz_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_submul_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sosq(
        z: *mut arf_struct,
        x: *mut arf_struct,
        y: *mut arf_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_div(
        z: arf_ptr,
        x: arf_srcptr,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_div_ui(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_ui_div(
        z: arf_ptr,
        x: mp_limb_t,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_div_si(
        z: arf_ptr,
        x: arf_srcptr,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_si_div(
        z: arf_ptr,
        x: mp_limb_signed_t,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_div_fmpz(
        z: arf_ptr,
        x: arf_srcptr,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_fmpz_div(
        z: arf_ptr,
        x: *mut fmpz,
        y: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_fmpz_div_fmpz(
        z: arf_ptr,
        x: *mut fmpz,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sqrt(
        z: arf_ptr,
        x: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sqrt_ui(
        z: *mut arf_struct,
        x: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sqrt_fmpz(
        z: *mut arf_struct,
        x: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_rsqrt(
        z: arf_ptr,
        x: arf_srcptr,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_root(
        z: *mut arf_struct,
        x: *mut arf_struct,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_get_mag(y: *mut mag_struct, x: *mut arf_struct);
    pub fn arf_get_mag_lower(y: *mut mag_struct, x: *mut arf_struct);
    pub fn arf_set_mag(y: *mut arf_struct, x: *mut mag_struct);
    pub fn mag_init_set_arf(y: *mut mag_struct, x: *mut arf_struct);
    pub fn mag_fast_init_set_arf(y: *mut mag_struct, x: *mut arf_struct);
    pub fn arf_mag_fast_add_ulp(
        z: *mut mag_struct,
        x: *mut mag_struct,
        y: *mut arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arf_mag_add_ulp(
        z: *mut mag_struct,
        x: *mut mag_struct,
        y: *mut arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arf_mag_set_ulp(z: *mut mag_struct, y: *mut arf_struct, prec: mp_limb_signed_t);
    pub fn arf_get_fmpq(y: *mut fmpq, x: *mut arf_struct);
    pub fn arf_set_fmpq(
        y: *mut arf_struct,
        x: *mut fmpq,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_complex_mul(
        e: *mut arf_struct,
        f: *mut arf_struct,
        a: *mut arf_struct,
        b: *mut arf_struct,
        c: *mut arf_struct,
        d: *mut arf_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_complex_mul_fallback(
        e: *mut arf_struct,
        f: *mut arf_struct,
        a: *mut arf_struct,
        b: *mut arf_struct,
        c: *mut arf_struct,
        d: *mut arf_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_complex_sqr(
        e: *mut arf_struct,
        f: *mut arf_struct,
        a: *mut arf_struct,
        b: *mut arf_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_sum(
        s: *mut arf_struct,
        terms: arf_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> c_int;
    pub fn arf_get_d(x: *mut arf_struct, rnd: c_int) -> f64;
    pub fn arf_set_d(x: *mut arf_struct, v: f64);
    pub fn arf_allocated_bytes(x: *mut arf_struct) -> mp_limb_signed_t;
    pub fn arf_load_str(
        res: *mut arf_struct,
        data: *const c_char,
    ) -> c_int;
    pub fn arf_dump_str(x: *mut arf_struct) -> *mut c_char;
    pub fn arf_load_file(res: *mut arf_struct, stream: *mut FILE) -> c_int;
    pub fn arf_dump_file(stream: *mut FILE, x: *mut arf_struct) -> c_int;
}
