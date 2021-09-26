# arb-sys

Rust bindings to the [Arb](https://arblib.org) library.

Arb is a C library for rigorous real and complex arithmetic with arbitrary precision.

## Binding Coverage

Most of Arb is covered, however some functions require `*mut` where `*const` will suffice (these need to be corrected but the bindings will still work as expected).

## Dependencies

This crate requires the Arb library to be installed:

  * [Arb](https://arblib.org) (version 2.21 recommended)
