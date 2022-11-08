#!/bin/bash

# Generates control and property definitions from yaml files
cargo run --bin generate_c > libcamera-sys/c_api/controls_generated.h
cargo run --bin generate_rust controls > libcamera-rs/src/generated/controls.rs
cargo run --bin generate_rust properties > libcamera-rs/src/generated/properties.rs
cargo fmt
