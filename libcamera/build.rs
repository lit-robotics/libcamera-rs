use std::{
    env, fs,
    path::{Path, PathBuf},
};

use semver::{Comparator, Op, Version};

struct LibcameraInfo {
    version: String,
    include_path: PathBuf,
}

fn get_libcamera_info() -> LibcameraInfo {
    // DEP_CAMERA_ vars are set by libcamera-sys build script (via links = "camera")
    let version = env::var("DEP_CAMERA_VERSION").expect("DEP_CAMERA_VERSION not set by libcamera-sys");
    let include_path =
        PathBuf::from(env::var("DEP_CAMERA_INCLUDE").expect("DEP_CAMERA_INCLUDE not set by libcamera-sys"));
    LibcameraInfo { version, include_path }
}

fn main() {
    println!("cargo:rustc-check-cfg=cfg(libcamera_has_vendor_controls)");

    let info = get_libcamera_info();

    let libcamera_version = match Version::parse(&info.version) {
        Ok(v) => v,
        Err(e) => {
            panic!("bad libcamera version '{}': {e:?}", info.version)
        }
    };

    let versioned_files = Path::new("versioned_files");
    let mut candidates = std::fs::read_dir(versioned_files)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let version = Version::parse(path.file_name()?.to_str()?).ok()?;

            Some((version, path))
        })
        .collect::<Vec<_>>();
    candidates.sort_unstable_by_key(|(version, _)| version.clone());

    // Filter to only compatible versions
    let matching = candidates.iter().filter(|(candidate, _)| {
        #[cfg(feature = "libcamera_semver_versioning")]
        let op = Op::Caret;
        #[cfg(not(feature = "libcamera_semver_versioning"))]
        let op = Op::Exact;

        let comparator = Comparator {
            op,
            major: candidate.major,
            minor: Some(candidate.minor),
            patch: Some(candidate.patch),
            pre: Default::default(),
        };

        comparator.matches(&libcamera_version) && candidate <= &libcamera_version
    });

    // And take the most recent compatible version
    let (_, selected_version) = match matching.max_by_key(|(version, _)| version.clone()) {
        Some(v) => v,
        None => panic!(
            "Unsupported version of libcamera detected: {libcamera_version}\nsupported versions are: \n{}",
            candidates
                .iter()
                .map(|(v, _)| format!("\t{v}"))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    for file in ["controls.rs", "properties.rs", "pixel_format_info.rs"] {
        std::fs::copy(selected_version.join(file), out_path.join(file)).unwrap();
        println!(
            "cargo:rerun-if-changed={}",
            selected_version.join(file).to_string_lossy()
        );
    }

    // Generate vendor feature flags from libcamera's generated control_ids.h
    let control_ids_header = info.include_path.join("libcamera").join("control_ids.h");
    let header_contents = fs::read_to_string(&control_ids_header).expect("Failed to read libcamera/control_ids.h");
    let mut vendor_controls_present = false;
    let mut feature_consts = String::new();
    for line in header_contents.lines() {
        if let Some(rest) = line.trim().strip_prefix("#define LIBCAMERA_HAS_") {
            let name = rest.split_whitespace().next().unwrap_or("").trim();
            if name.is_empty() {
                continue;
            }
            feature_consts.push_str(&format!("pub const LIBCAMERA_HAS_{}: bool = true;\n", name));
            if name.contains("LIBCAMERA_VENDOR_CONTROLS") {
                vendor_controls_present = true;
            }
        }
    }
    let features_rs = out_path.join("vendor_features.rs");
    fs::write(&features_rs, feature_consts).expect("Failed to write vendor_features.rs");
    println!("cargo:rerun-if-changed={}", control_ids_header.to_string_lossy());
    if vendor_controls_present {
        println!("cargo:rustc-cfg=libcamera_has_vendor_controls");
    }

    // Generate pixel format constants from libcamera/formats.h
    let formats_header = info.include_path.join("libcamera").join("formats.h");
    let formats_contents = fs::read_to_string(&formats_header).expect("Failed to read libcamera/formats.h");
    let mut generated = String::from(
        "// Auto-generated from libcamera/formats.h\n\
#[allow(unused_imports)]\n\
use crate::pixel_format::PixelFormat;\n\n",
    );
    for line in formats_contents.lines() {
        let line = line.trim();
        if !line.starts_with("constexpr PixelFormat") {
            continue;
        }
        let name_start = "constexpr PixelFormat ".len();
        let name_end = match line[name_start..].find('(') {
            Some(idx) => name_start + idx,
            None => continue,
        };
        let name = &line[name_start..name_end];
        let rest = &line[name_end + 1..];
        let parts: Vec<&str> = rest.split("__mod(").collect();
        if parts.len() < 2 {
            continue;
        }
        let fourcc_part = parts[0].trim_end_matches(", ");
        let mod_part = parts[1];

        let big_endian = fourcc_part.contains("kDrmFormatBigEndian");
        let fourcc_inner_start = "__fourcc(".len();
        let fourcc_inner = &fourcc_part[fourcc_inner_start..]
            .trim_start_matches("__fourcc(")
            .trim_end_matches(')')
            .trim();
        let chars: Vec<u32> = fourcc_inner
            .split(',')
            .filter_map(|c| c.trim().trim_matches('\'').chars().next().map(|ch| ch as u32))
            .collect();
        if chars.len() != 4 {
            continue;
        }
        let mut fourcc: u32 = chars[0] | (chars[1] << 8) | (chars[2] << 16) | (chars[3] << 24);
        if big_endian {
            fourcc |= 1u32 << 31;
        }

        let mod_inner = mod_part.split(')').next().unwrap_or("").trim().trim_start_matches('(');
        let mut mod_nums = mod_inner.split(',').map(|s| s.trim().parse::<u64>().unwrap_or(0));
        let vendor = mod_nums.next().unwrap_or(0);
        let modifier = mod_nums.next().unwrap_or(0);
        let modifier = (vendor << 56) | modifier;

        generated.push_str(&format!(
            "pub const {name}: PixelFormat = PixelFormat::new(0x{fourcc:08x}, 0x{modifier:016x});\n",
        ));
    }
    let formats_rs = out_path.join("formats.rs");
    fs::write(&formats_rs, generated).expect("Failed to write formats.rs");
    println!("cargo:rerun-if-changed={}", formats_header.to_string_lossy());
}
