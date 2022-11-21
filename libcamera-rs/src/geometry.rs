use libcamera_sys::*;

/// Represents `libcamera::Point`
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<libcamera_point_t> for Point {
    fn from(p: libcamera_point_t) -> Self {
        Self { x: p.x, y: p.y }
    }
}

/// Represents `libcamera::Size`
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl From<libcamera_size_t> for Size {
    fn from(s: libcamera_size_t) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

impl From<Size> for libcamera_size_t {
    fn from(s: Size) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

/// Represents `libcamera::SizeRange`
#[derive(Debug, Clone, Copy)]
pub struct SizeRange {
    pub min: Size,
    pub max: Size,
    pub h_step: u32,
    pub v_step: u32,
}

impl From<libcamera_size_range_t> for SizeRange {
    fn from(r: libcamera_size_range_t) -> Self {
        Self {
            min: r.min.into(),
            max: r.max.into(),
            h_step: r.hStep,
            v_step: r.vStep,
        }
    }
}

impl From<SizeRange> for libcamera_size_range_t {
    fn from(r: SizeRange) -> Self {
        Self {
            min: r.min.into(),
            max: r.max.into(),
            hStep: r.h_step,
            vStep: r.v_step,
        }
    }
}

/// Represents `libcamera::Rectangle`
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl From<libcamera_rectangle_t> for Rectangle {
    fn from(r: libcamera_rectangle_t) -> Self {
        Self {
            x: r.x,
            y: r.y,
            width: r.width,
            height: r.height,
        }
    }
}

impl From<Rectangle> for libcamera_rectangle_t {
    fn from(r: Rectangle) -> Self {
        Self {
            x: r.x,
            y: r.y,
            width: r.width,
            height: r.height,
        }
    }
}
