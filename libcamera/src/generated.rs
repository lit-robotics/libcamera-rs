// Ignore documentation formatting clippy lints in generated files
#![allow(clippy::doc_lazy_continuation)]

pub mod controls {
    include!(concat!(env!("OUT_DIR"), "/controls.rs"));
}

pub mod properties {
    include!(concat!(env!("OUT_DIR"), "/properties.rs"));
}
