#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! *See the [Arb documentation](https://arblib.org/).

use flint_sys::deps::*;
use flint_sys::fmpz::fmpz;
use crate::mag::mag_struct;
use crate::arf::arf_struct;
use crate::arb::{arb_struct, arb_ptr, arb_srcptr};
use crate::acb::{acb_struct, acb_t, acb_ptr, acb_srcptr};
use crate::acb_poly::acb_poly_struct;
use crate::dirichlet::{dirichlet_group_struct, dirichlet_char_struct};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acb_dirichlet_hurwitz_precomp_struct {
    pub s: acb_struct,
    pub err: mag_struct,
    pub coeffs: acb_ptr,
    pub deflate: ::std::os::raw::c_int,
    pub A: mp_limb_signed_t,
    pub N: mp_limb_signed_t,
    pub K: mp_limb_signed_t,
}

pub type acb_dirichlet_hurwitz_precomp_t = [acb_dirichlet_hurwitz_precomp_struct; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acb_dirichlet_roots_struct {
    pub order: mp_limb_t,
    pub reduced_order: mp_limb_t,
    pub z: acb_t,
    pub size: mp_limb_signed_t,
    pub depth: mp_limb_signed_t,
    pub Z: *mut acb_ptr,
    pub use_pow: ::std::os::raw::c_int,
}

pub type acb_dirichlet_roots_t = [acb_dirichlet_roots_struct; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acb_dirichlet_platt_c_precomp_struct {
    pub len: mp_limb_signed_t,
    pub p: arb_ptr,
    pub Xa: arb_struct,
    pub Xb: arb_struct,
}

pub type acb_dirichlet_platt_c_precomp_t = [acb_dirichlet_platt_c_precomp_struct; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acb_dirichlet_platt_i_precomp_struct {
    pub c1: arb_struct,
    pub c2: arb_struct,
}

pub type acb_dirichlet_platt_i_precomp_t = [acb_dirichlet_platt_i_precomp_struct; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acb_dirichlet_platt_ws_precomp_struct {
    pub pre_c: acb_dirichlet_platt_c_precomp_struct,
    pub pre_i: acb_dirichlet_platt_i_precomp_struct,
}

pub type acb_dirichlet_platt_ws_precomp_t = [acb_dirichlet_platt_ws_precomp_struct; 1usize];

extern "C" {
    pub fn acb_dirichlet_powsum_term(
        res: acb_ptr,
        log_prev: *mut arb_struct,
        prev: *mut mp_limb_t,
        s: *mut acb_struct,
        k: mp_limb_t,
        integer: ::std::os::raw::c_int,
        critical_line: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_powsum_sieved(
        z: acb_ptr,
        s: *mut acb_struct,
        n: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_powsum_smooth(
        z: acb_ptr,
        s: *mut acb_struct,
        n: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_zeta_bound(res: *mut mag_struct, s: *mut acb_struct);
    pub fn acb_dirichlet_zeta_deriv_bound(
        der1: *mut mag_struct,
        der2: *mut mag_struct,
        s: *mut acb_struct,
    );
    pub fn acb_dirichlet_zeta_rs_f_coeffs(
        c: acb_ptr,
        p: *mut arb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_zeta_rs_d_coeffs(
        d: arb_ptr,
        sigma: *mut arb_struct,
        k: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_zeta_rs_bound(
        err: *mut mag_struct,
        s: *mut acb_struct,
        K: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_zeta_rs_r(
        res: *mut acb_struct,
        s: *mut acb_struct,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_zeta_rs(
        res: *mut acb_struct,
        s: *mut acb_struct,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_zeta(res: *mut acb_struct, s: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_dirichlet_zeta_jet_rs(
        res: acb_ptr,
        s: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_zeta_jet(
        res: *mut acb_struct,
        s: *mut acb_struct,
        deflate: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hurwitz(
        res: *mut acb_struct,
        s: *mut acb_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_stieltjes(
        res: *mut acb_struct,
        n: *mut fmpz,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hurwitz_precomp_init(
        pre: *mut acb_dirichlet_hurwitz_precomp_struct,
        s: *mut acb_struct,
        deflate: ::std::os::raw::c_int,
        A: mp_limb_signed_t,
        K: mp_limb_signed_t,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hurwitz_precomp_init_num(
        pre: *mut acb_dirichlet_hurwitz_precomp_struct,
        s: *mut acb_struct,
        deflate: ::std::os::raw::c_int,
        num_eval: f64,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hurwitz_precomp_clear(pre: *mut acb_dirichlet_hurwitz_precomp_struct);
    pub fn acb_dirichlet_hurwitz_precomp_bound(
        res: *mut mag_struct,
        s: *mut acb_struct,
        A: mp_limb_signed_t,
        K: mp_limb_signed_t,
        N: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hurwitz_precomp_eval(
        res: *mut acb_struct,
        pre: *mut acb_dirichlet_hurwitz_precomp_struct,
        p: mp_limb_t,
        q: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hurwitz_precomp_choose_param(
        A: *mut mp_limb_t,
        K: *mut mp_limb_t,
        N: *mut mp_limb_t,
        s: *mut acb_struct,
        num_eval: f64,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dirichlet_euler_product_real_ui(
        res: *mut arb_struct,
        s: mp_limb_t,
        chi: *const ::std::os::raw::c_schar,
        mod_: ::std::os::raw::c_int,
        reciprocal: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_eta(res: *mut acb_struct, s: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_dirichlet_xi(res: *mut acb_struct, s: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_dirichlet_pairing(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        m: mp_limb_t,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_pairing_char(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        a: *mut dirichlet_char_struct,
        b: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_roots_init(
        t: *mut acb_dirichlet_roots_struct,
        order: mp_limb_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_roots_clear(t: *mut acb_dirichlet_roots_struct);
    pub fn acb_dirichlet_root(
        z: *mut acb_struct,
        t: *mut acb_dirichlet_roots_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_chi(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_chi_vec(
        v: acb_ptr,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        nv: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_arb_quadratic_powers(
        v: arb_ptr,
        nv: mp_limb_signed_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_qseries_arb(
        res: *mut acb_struct,
        a: acb_srcptr,
        x: *mut arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_qseries_arb_powers_naive(
        res: *mut acb_struct,
        x: *mut arb_struct,
        parity: ::std::os::raw::c_int,
        a: *const mp_limb_t,
        z: *mut acb_dirichlet_roots_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_qseries_arb_powers_smallorder(
        res: *mut acb_struct,
        x: *mut arb_struct,
        parity: ::std::os::raw::c_int,
        a: *const mp_limb_t,
        z: *mut acb_dirichlet_roots_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_theta_length_d(q: mp_limb_t, x: f64, prec: mp_limb_signed_t) -> mp_limb_t;
    pub fn acb_dirichlet_theta_length(
        q: mp_limb_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    ) -> mp_limb_t;
    pub fn mag_tail_kexpk2_arb(res: *mut mag_struct, a: *mut arb_struct, n: mp_limb_t);
    pub fn _acb_dirichlet_theta_argument_at_arb(
        xt: *mut arb_struct,
        q: mp_limb_t,
        t: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_theta_arb(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        t: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_ui_theta_arb(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        a: mp_limb_t,
        t: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_gauss_sum_naive(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_gauss_sum_factor(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_gauss_sum_order2(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_gauss_sum_theta(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_gauss_sum(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_root_number_theta(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_root_number(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_si_poly_evaluate(
        res: *mut acb_struct,
        v: *mut mp_limb_signed_t,
        len: mp_limb_signed_t,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_jacobi_sum_naive(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi1: *mut dirichlet_char_struct,
        chi2: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn jacobi_one_prime(
        p: mp_limb_t,
        e: mp_limb_t,
        pe: mp_limb_t,
        cond: mp_limb_t,
    ) -> mp_limb_t;
    pub fn acb_dirichlet_jacobi_sum_factor(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi1: *mut dirichlet_char_struct,
        chi2: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_jacobi_sum_gauss(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi1: *mut dirichlet_char_struct,
        chi2: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_jacobi_sum(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi1: *mut dirichlet_char_struct,
        chi2: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_jacobi_sum_ui(
        res: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        a: mp_limb_t,
        b: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_l_euler_product(
        res: *mut acb_struct,
        s: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_l_hurwitz(
        res: *mut acb_struct,
        s: *mut acb_struct,
        precomp: *mut acb_dirichlet_hurwitz_precomp_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_l(
        res: *mut acb_struct,
        s: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_l_vec_hurwitz(
        res: acb_ptr,
        s: *mut acb_struct,
        precomp: *mut acb_dirichlet_hurwitz_precomp_struct,
        G: *mut dirichlet_group_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_l_jet(
        res: acb_ptr,
        s: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        deflate: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dirichlet_l_series(
        res: acb_ptr,
        s: acb_srcptr,
        slen: mp_limb_signed_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        deflate: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_l_series(
        res: *mut acb_poly_struct,
        s: *mut acb_poly_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        deflate: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hardy_theta(
        res: acb_ptr,
        t: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hardy_z(
        res: acb_ptr,
        t: *mut acb_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dirichlet_hardy_theta_series(
        res: acb_ptr,
        s: acb_srcptr,
        slen: mp_limb_signed_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hardy_theta_series(
        res: *mut acb_poly_struct,
        s: *mut acb_poly_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dirichlet_hardy_z_series(
        res: acb_ptr,
        s: acb_srcptr,
        slen: mp_limb_signed_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hardy_z_series(
        res: *mut acb_poly_struct,
        s: *mut acb_poly_struct,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_gram_point(
        res: *mut arb_struct,
        n: *mut fmpz,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_turing_method_bound(p: *mut fmpz) -> mp_limb_t;
    pub fn _acb_dirichlet_definite_hardy_z(
        res: *mut arb_struct,
        t: *mut arf_struct,
        pprec: *mut mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn _acb_dirichlet_isolate_gram_hardy_z_zero(
        a: *mut arf_struct,
        b: *mut arf_struct,
        n: *mut fmpz,
    );
    pub fn _acb_dirichlet_isolate_rosser_hardy_z_zero(
        a: *mut arf_struct,
        b: *mut arf_struct,
        n: *mut fmpz,
    );
    pub fn _acb_dirichlet_isolate_turing_hardy_z_zero(
        a: *mut arf_struct,
        b: *mut arf_struct,
        n: *mut fmpz,
    );
    pub fn acb_dirichlet_isolate_hardy_z_zero(a: *mut arf_struct, b: *mut arf_struct, n: *mut fmpz);
    pub fn _acb_dirichlet_refine_hardy_z_zero(
        res: *mut arb_struct,
        a: *mut arf_struct,
        b: *mut arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_hardy_z_zeros(
        res: arb_ptr,
        n: *mut fmpz,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_zeta_zeros(
        res: acb_ptr,
        n: *mut fmpz,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_zeta_zeros(
        res: acb_ptr,
        n: *mut fmpz,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn _acb_dirichlet_exact_zeta_nzeros(res: *mut fmpz, t: *mut arf_struct);
    pub fn acb_dirichlet_zeta_nzeros(
        res: *mut arb_struct,
        t: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_backlund_s(
        res: *mut arb_struct,
        t: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_backlund_s_bound(res: *mut mag_struct, t: *mut arb_struct);
    pub fn acb_dirichlet_zeta_nzeros_gram(res: *mut fmpz, n: *mut fmpz);
    pub fn acb_dirichlet_backlund_s_gram(n: *mut fmpz) -> mp_limb_signed_t;
    pub fn acb_dirichlet_platt_c_precomp_init(
        pre: *mut acb_dirichlet_platt_c_precomp_struct,
        sigma: mp_limb_signed_t,
        h: *mut arb_struct,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_c_precomp_clear(pre: *mut acb_dirichlet_platt_c_precomp_struct);
    pub fn acb_dirichlet_platt_c_bound_precomp(
        res: *mut arb_struct,
        pre: *mut acb_dirichlet_platt_c_precomp_struct,
        sigma: mp_limb_signed_t,
        t0: *mut arb_struct,
        h: *mut arb_struct,
        k: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_c_bound(
        res: *mut arb_struct,
        sigma: mp_limb_signed_t,
        t0: *mut arb_struct,
        h: *mut arb_struct,
        k: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_i_precomp_init(
        pre: *mut acb_dirichlet_platt_i_precomp_struct,
        A: mp_limb_signed_t,
        H: *mut arb_struct,
        sigma: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_i_precomp_clear(pre: *mut acb_dirichlet_platt_i_precomp_struct);
    pub fn acb_dirichlet_platt_i_bound_precomp(
        res: *mut arb_struct,
        pre_i: *mut acb_dirichlet_platt_i_precomp_struct,
        pre_c: *mut acb_dirichlet_platt_c_precomp_struct,
        t0: *mut arb_struct,
        A: mp_limb_signed_t,
        H: *mut arb_struct,
        sigma: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_i_bound(
        res: *mut arb_struct,
        t0: *mut arb_struct,
        A: mp_limb_signed_t,
        H: *mut arb_struct,
        sigma: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_ws_precomp_init(
        pre: *mut acb_dirichlet_platt_ws_precomp_struct,
        A: mp_limb_signed_t,
        H: *mut arb_struct,
        sigma: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_ws_precomp_clear(pre: *mut acb_dirichlet_platt_ws_precomp_struct);
    pub fn acb_dirichlet_platt_ws_interpolation_precomp(
        res: *mut arb_struct,
        deriv: *mut arf_struct,
        pre: *mut acb_dirichlet_platt_ws_precomp_struct,
        t0: *mut arb_struct,
        p: arb_srcptr,
        T: *mut fmpz,
        A: mp_limb_signed_t,
        B: mp_limb_signed_t,
        Ns_max: mp_limb_signed_t,
        H: *mut arb_struct,
        sigma: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_ws_interpolation(
        res: *mut arb_struct,
        deriv: *mut arf_struct,
        t0: *mut arb_struct,
        p: arb_srcptr,
        T: *mut fmpz,
        A: mp_limb_signed_t,
        B: mp_limb_signed_t,
        Ns_max: mp_limb_signed_t,
        H: *mut arb_struct,
        sigma: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_bound_C3(
        res: *mut arb_struct,
        t0: *mut arb_struct,
        A: mp_limb_signed_t,
        H: *mut arb_struct,
        Ns: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_scaled_lambda(
        res: *mut arb_struct,
        t: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_scaled_lambda_vec(
        res: arb_ptr,
        T: *mut fmpz,
        A: mp_limb_signed_t,
        B: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_beta(
        res: *mut arb_struct,
        t: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_lemma_32(
        out: *mut arb_struct,
        h: *mut arb_struct,
        t0: *mut arb_struct,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_lemma_A5(
        out: *mut arb_struct,
        B: mp_limb_signed_t,
        h: *mut arb_struct,
        k: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_lemma_A7(
        out: *mut arb_struct,
        sigma: mp_limb_signed_t,
        t0: *mut arb_struct,
        h: *mut arb_struct,
        k: mp_limb_signed_t,
        A: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_lemma_A9(
        out: *mut arb_struct,
        sigma: mp_limb_signed_t,
        t0: *mut arb_struct,
        h: *mut arb_struct,
        A: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_lemma_A11(
        out: *mut arb_struct,
        t0: *mut arb_struct,
        h: *mut arb_struct,
        B: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_lemma_B1(
        out: *mut arb_struct,
        sigma: mp_limb_signed_t,
        t0: *mut arb_struct,
        h: *mut arb_struct,
        J: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_lemma_B2(
        out: *mut arb_struct,
        K: mp_limb_signed_t,
        h: *mut arb_struct,
        xi: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_multieval(
        out: arb_ptr,
        T: *mut fmpz,
        A: mp_limb_signed_t,
        B: mp_limb_signed_t,
        h: *mut arb_struct,
        J: mp_limb_signed_t,
        K: mp_limb_signed_t,
        sigma: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_platt_multieval_threaded(
        out: arb_ptr,
        T: *mut fmpz,
        A: mp_limb_signed_t,
        B: mp_limb_signed_t,
        h: *mut arb_struct,
        J: mp_limb_signed_t,
        K: mp_limb_signed_t,
        sigma: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dirichlet_platt_local_hardy_z_zeros(
        res: arb_ptr,
        n: *mut fmpz,
        len: mp_limb_signed_t,
        T: *mut fmpz,
        A: mp_limb_signed_t,
        B: mp_limb_signed_t,
        h: *mut arb_struct,
        J: mp_limb_signed_t,
        K: mp_limb_signed_t,
        sigma_grid: mp_limb_signed_t,
        Ns_max: mp_limb_signed_t,
        H: *mut arb_struct,
        sigma_interp: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_dirichlet_platt_local_hardy_z_zeros(
        res: arb_ptr,
        n: *mut fmpz,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_dirichlet_platt_hardy_z_zeros(
        res: arb_ptr,
        n: *mut fmpz,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_dirichlet_dft_index(
        w: acb_ptr,
        v: acb_srcptr,
        G: *mut dirichlet_group_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dirichlet_dft(
        w: acb_ptr,
        v: acb_srcptr,
        G: *mut dirichlet_group_struct,
        prec: mp_limb_signed_t,
    );
}
