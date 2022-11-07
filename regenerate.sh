#!/bin/bash

# Generates control and property definitions from yaml files
cargo run --bin generate_c > libcamera-sys/c_api/controls_generated.h
cargo run --bin generate_rust > libcamera-rs/src/controls_generated.rs
cargo fmt
