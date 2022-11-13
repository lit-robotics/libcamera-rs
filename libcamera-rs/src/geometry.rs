use libcamera_sys::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<libcamera_point_t> for Point {
    fn from(p: libcamera_point_t) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
