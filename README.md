# arb-sys

Rust bindings to the [Arb](https://arblib.org) library.

Arb is a C library for rigorous real and complex arithmetic with arbitrary precision.

## Optional features

  * `disable-make-check`: this can reduce compilation time significantly.

## Notes
  
  * As of version 0.3.0 the Arb source files are now included and the library is compiled automatically. The files are cached to avoid unnecessary compilations.

  * Some binding arguments may be marked mutable instead of const. See 
  [issue 2](https://github.com/wjyoumans/arb-sys/issues/2) for details. They are being manually 
  updated over time, but feel free to correct any bindings and make a pull request if you would like
  certain ones fixed right away.
