#!/bin/bash

cargo run --bin generate_from_git

cargo +nightly fmt --all
