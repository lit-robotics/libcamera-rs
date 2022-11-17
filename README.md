# libcamera-rs

Experimental Rust bindings for [libcamera](https://libcamera.org/).

Project structure:
  - [libcamera-sys](./libcamera-sys/) - Low-level unsafe bindings to libcamera. Also contains libcamera [C API shim](./libcamera-sys/c_api/) to make interfacing with C++ code easier.
  - [libcamera-meta](./libcamera-meta/) - Scripts for generating C and Rust code from libcamera controls, properties and formats YAMLs. Mostly used by the [regenerate.sh](./regenerate.sh) script.
  - [libcamera-rs](./libcamera-rs/) - Safe libcamera Rust interface on top of `libcamera-sys`.

