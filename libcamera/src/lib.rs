#![warn(rust_2018_idioms)]

pub mod camera;
pub mod camera_manager;
pub mod control;
pub mod control_value;
pub mod framebuffer;
pub mod framebuffer_allocator;
pub mod framebuffer_map;
pub mod geometry;
pub mod logging;
pub mod pixel_format;
pub mod request;
pub mod stream;
pub mod utils;

mod generated;
pub use generated::*;
