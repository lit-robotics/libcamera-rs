use std::{
    env,
    path::{Path, PathBuf},
};

use semver::{Comparator, Op, Version};

fn main() {
    let libcamera_version = Version::new(
        libcamera_sys::LIBCAMERA_VERSION_MAJOR as _,
        libcamera_sys::LIBCAMERA_VERSION_MINOR as _,
        libcamera_sys::LIBCAMERA_VERSION_PATCH as _,
    );

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

        comparator.matches(&libcamera_version)
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

    for file in ["controls.rs", "properties.rs"] {
        std::fs::copy(selected_version.join(file), out_path.join(file)).unwrap();
    }
}
