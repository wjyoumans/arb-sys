//! Bindings for the [ARB](https://arblib.org/) library.
//! Crates marked with an asterisk have functions which may require mutable borrows where const
//! borrows will suffice (these need to be corrected but the bindings will still work as expected).

//pub mod deps;
//pub mod flint;

pub mod fmpr;
pub mod mag;
pub mod arf;
pub mod arb;
pub mod arb_calc;
pub mod arb_poly;
pub mod arb_fmpz_poly;
pub mod arb_hypgeom;
pub mod arb_mat;
pub mod acb;
pub mod acb_calc;
pub mod acb_poly;
pub mod acb_dft;
pub mod acb_dirichlet;
pub mod acb_elliptic;
pub mod acb_hypgeom;
pub mod acb_modular;
pub mod acb_mat;
pub mod arith;
pub mod bernoulli;
pub mod bool_mat;
pub mod dirichlet;
pub mod dlog;
pub mod hypgeom;
pub mod partition;
