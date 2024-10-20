use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let version = format!(
        "{}.{}.{}",
        libcamera_sys::LIBCAMERA_VERSION_MAJOR,
        libcamera_sys::LIBCAMERA_VERSION_MINOR,
        libcamera_sys::LIBCAMERA_VERSION_PATCH
    );

    let versioned_files = Path::new("versioned_files").join(&version);

    if std::fs::metadata(&versioned_files).is_err() {
        panic!("Unsupported version of libcamera detected: {version}");
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    for file in ["controls.rs", "properties.rs"] {
        std::fs::copy(versioned_files.join(file), out_path.join(file)).unwrap();
    }
}
