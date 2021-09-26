#![allow(non_camel_case_types)]

//! *See the [ARB documentation](https://arblib.org/).

use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpz::fmpz;
use flint_sys::fmpq::fmpq;
use crate::fmpr::fmpr_struct;
use libc::{c_char, c_int, FILE};
