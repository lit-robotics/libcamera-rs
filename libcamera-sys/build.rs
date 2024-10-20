use std::{env, fs, path::PathBuf};

fn main() {
    let libcamera = match pkg_config::probe_library("libcamera") {
        Ok(lib) => Ok(lib),
        Err(e) => {
            // Older libcamera versions use camera name instead of libcamera, try that instead
            match pkg_config::probe_library("camera") {
                Ok(lib) => Ok(lib),
                // Return original error
                Err(_) => Err(e),
            }
        }
    }
    .unwrap();

    let libcamera_include_path = libcamera
        .include_paths
        .get(0)
        .expect("Unable to get libcamera include path");

    println!("cargo:rustc-link-lib=camera");

    let mut c_api_headers: Vec<PathBuf> = Vec::new();
    let mut cpp_api_headers: Vec<PathBuf> = Vec::new();
    let mut c_api_sources: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir("c_api").unwrap() {
        let entry = entry.unwrap();

        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        match entry.path().extension().and_then(|s| s.to_str()) {
            Some("h") => c_api_headers.push(entry.path()),
            Some("hpp") => cpp_api_headers.push(entry.path()),
            Some("cpp") => c_api_sources.push(entry.path()),
            _ => {}
        }
    }

    for file in c_api_headers
        .iter()
        .chain(cpp_api_headers.iter())
        .chain(c_api_sources.iter())
    {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .files(c_api_sources)
        .include(libcamera_include_path)
        .compile("camera_c_api");

    // C bindings
    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", libcamera_include_path.display()))
        .constified_enum_module("libcamera_.*")
        .allowlist_function("libcamera_.*")
        .allowlist_var("LIBCAMERA_.*")
        .allowlist_var(".*LIBCAMERA_VERSION.*")
        .allowlist_type("libcamera_.*");
    for header in c_api_headers {
        builder = builder.header(header.to_str().unwrap());
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // CPP bindings
    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", libcamera_include_path.display()))
        .clang_arg("-std=c++17")
        .allowlist_type(".*controls.*")
        .allowlist_type(".*properties.*");
    for header in cpp_api_headers {
        builder = builder.header(header.to_str().unwrap());
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings_cpp.rs"))
        .expect("Couldn't write bindings!");
}
