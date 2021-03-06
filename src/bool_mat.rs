#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpz_mat::fmpz_mat_struct;
use libc::FILE;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bool_mat_struct {
    pub entries: *mut ::std::os::raw::c_int,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut ::std::os::raw::c_int,
}

pub type bool_mat_t = [bool_mat_struct; 1usize];

extern "C" {
    pub fn bool_mat_init(mat: *mut bool_mat_struct, r: mp_limb_signed_t, c: mp_limb_signed_t);
    pub fn bool_mat_clear(mat: *mut bool_mat_struct);
    pub fn bool_mat_set(dest: *mut bool_mat_struct, src: *mut bool_mat_struct);
    pub fn bool_mat_randtest(mat: *mut bool_mat_struct, state: *mut flint_rand_s);
    pub fn bool_mat_randtest_diagonal(mat: *mut bool_mat_struct, state: *mut flint_rand_s);
    pub fn bool_mat_randtest_nilpotent(mat: *mut bool_mat_struct, state: *mut flint_rand_s);
    pub fn bool_mat_fprint(file: *mut FILE, mat: *mut bool_mat_struct);
    pub fn bool_mat_equal(
        mat1: *mut bool_mat_struct,
        mat2: *mut bool_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn bool_mat_any(mat: *mut bool_mat_struct) -> ::std::os::raw::c_int;
    pub fn bool_mat_all(mat: *mut bool_mat_struct) -> ::std::os::raw::c_int;
    pub fn bool_mat_is_diagonal(mat: *mut bool_mat_struct) -> ::std::os::raw::c_int;
    pub fn bool_mat_is_lower_triangular(mat: *mut bool_mat_struct) -> ::std::os::raw::c_int;
    pub fn bool_mat_is_transitive(mat: *mut bool_mat_struct) -> ::std::os::raw::c_int;
    pub fn bool_mat_is_nilpotent(mat: *mut bool_mat_struct) -> ::std::os::raw::c_int;
    pub fn bool_mat_zero(mat: *mut bool_mat_struct);
    pub fn bool_mat_one(mat: *mut bool_mat_struct);
    pub fn bool_mat_directed_path(mat: *mut bool_mat_struct);
    pub fn bool_mat_directed_cycle(mat: *mut bool_mat_struct);
    pub fn bool_mat_transpose(mat1: *mut bool_mat_struct, mat2: *mut bool_mat_struct);
    pub fn bool_mat_complement(mat1: *mut bool_mat_struct, mat2: *mut bool_mat_struct);
    pub fn bool_mat_add(
        res: *mut bool_mat_struct,
        mat1: *mut bool_mat_struct,
        mat2: *mut bool_mat_struct,
    );
    pub fn bool_mat_mul(
        res: *mut bool_mat_struct,
        mat1: *mut bool_mat_struct,
        mat2: *mut bool_mat_struct,
    );
    pub fn bool_mat_mul_entrywise(
        res: *mut bool_mat_struct,
        mat1: *mut bool_mat_struct,
        mat2: *mut bool_mat_struct,
    );
    pub fn bool_mat_pow_ui(B: *mut bool_mat_struct, A: *mut bool_mat_struct, exp: mp_limb_t);
    pub fn bool_mat_trace(mat: *mut bool_mat_struct) -> ::std::os::raw::c_int;
    pub fn bool_mat_nilpotency_degree(mat: *mut bool_mat_struct) -> mp_limb_signed_t;
    pub fn bool_mat_transitive_closure(dest: *mut bool_mat_struct, src: *mut bool_mat_struct);
    pub fn bool_mat_get_strongly_connected_components(
        partition: *mut mp_limb_signed_t,
        A: *mut bool_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn bool_mat_all_pairs_longest_walk(
        B: *mut fmpz_mat_struct,
        A: *mut bool_mat_struct,
    ) -> mp_limb_signed_t;
}
