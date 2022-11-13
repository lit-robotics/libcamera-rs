use std::ffi::CStr;

use libcamera_sys::*;

fn main() {
    unsafe {
        let mgr = libcamera_camera_manager_create();

        let version = CStr::from_ptr(libcamera_camera_manager_version(mgr)).to_str().unwrap();
        println!("libcamera: {}", version);

        libcamera_camera_manager_destroy(mgr);
    }
}
