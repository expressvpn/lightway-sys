# lightway-sys
This crate provides auto-generated unsafe Rust bindings through [bindgen](https://github.com/rust-lang/rust-bindgen/), to C functions provided by the Lightway Core library.


*Note*: This is a very early release and as such there are a number of limitations with this implementation. Expect it to improve significantly in the very near future.


## Getting started
Add `lightway-sys` to your Cargo manifest:

```
[dependencies]
lightway-sys = "0.1.2"
```

To ensure that the crate can be built even offline, the crate includes the source code for Lightway (currently version `1.7.0`).

## Building with Earthly
There is also an `Earthfile` provided so that you can build the crate in [Earthly](https://earthly.dev):

```
earthly +build-crate
```

## TODO

* Resolve the warnings in the library build phase

