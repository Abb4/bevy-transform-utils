# bevy-transform-utils
Collection of useful util functions for bevy transforms.

## Compiling without bevy
Bevy uses glam for its core types and the compilation time can be shortened significantly if bevy imports are avoided.
If you don't need bevy, include/compile this crate with the `nobevytypes` [feature](https://doc.rust-lang.org/cargo/reference/features.html) by running `cargo build --features 'nobevytypes'`.