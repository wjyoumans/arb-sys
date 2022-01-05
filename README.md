# arb-sys

Rust bindings to the [Arb](https://arblib.org) library.

Arb is a C library for rigorous real and complex arithmetic with arbitrary precision.

## Dependencies

This crate requires the Arb library to be installed:

  * [Arb](https://arblib.org) (version >= 2.21 recommended)

## Notes

  * Some binding arguments may be marked mutable instead of const. See 
  [issue 2](https://github.com/wjyoumans/arb-sys/issues/2) for details. They are being manually 
  updated over time, but feel free to correct any bindings and make a pull request if you would like
  certain ones fixed right away.
  * On Debian the Arb C library may be named `flint-arb`, in which case you will need to edit the 
  `build.rs` line `println!("cargo:rustc-link-lib=arb");` to 
  `println!("cargo:rustc-link-lib=flint-arb");` (thanks to Christophe Troestler for pointing this 
  out).
