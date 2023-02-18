#!/bin/bash

# Generates control and property definitions from yaml files
cargo run --bin generate_c > libcamera-sys/c_api/controls_generated.h
# This could be automated with a procedural macro, but it makes code hard to read and explore.
cargo run --bin generate_rust controls > libcamera/src/generated/controls.rs
cargo run --bin generate_rust properties > libcamera/src/generated/properties.rs
cargo fmt
