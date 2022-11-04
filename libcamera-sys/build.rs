use std::{env, path::PathBuf};

fn main() {
    let libcamera = pkg_config::find_library("libcamera").unwrap();
    let include_path = libcamera
        .include_paths
        .get(0)
        .expect("Unable to get libcamera include path");

    println!("cargo:rustc-link-lib=camera");

    let headers = [
        "libcamera/c_api/camera_manager.h",
        "libcamera/c_api/camera.h",
    ];

    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include_path.display()))
        .allowlist_type("regex")
        .allowlist_function("libcamera_.*")
        .allowlist_var("LIBCAMERA_.*")
        .allowlist_type("libcamera_.*");

    for header in headers {
        builder = builder.header(include_path.join(header).display().to_string());
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
