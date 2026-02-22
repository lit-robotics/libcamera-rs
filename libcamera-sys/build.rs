use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};

/// Information about the discovered or built library.
struct Library {
    include_paths: Vec<PathBuf>,
}

/// Represents the link kind for the library.
#[derive(Clone, Copy)]
enum LinkKind {
    /// Use the default (static for vendored, whatever pkg-config says otherwise).
    Default,
    /// Force static linking.
    Static,
    /// Force dynamic linking.
    Dynamic,
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LIBCAMERA_SOURCE");
    println!("cargo:rerun-if-env-changed=LIBCAMERA_STATIC");
    println!("cargo:rerun-if-env-changed=LIBCAMERA_DYNAMIC");
    println!("cargo:rerun-if-env-changed=LIBCAMERA_LIB_DIR");
    println!("cargo:rerun-if-env-changed=LIBCAMERA_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=LIBCAMERA_PIPELINES");

    let link_kind = get_link_kind();
    let library = build_or_find_library(link_kind)?;
    compile_c_api(&library)?;
    generate_bindings(&library)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Environment variable helpers (supports target-triple prefix)
// ---------------------------------------------------------------------------

/// Get the target triple, uppercased with hyphens replaced by underscores.
fn target_triple_prefix() -> String {
    env::var("TARGET").unwrap_or_default().to_uppercase().replace('-', "_")
}

/// Read an env var with optional target-triple prefix override.
/// Checks `<TRIPLE>_<NAME>` first, then `<NAME>`.
fn get_env(name: &str) -> Option<String> {
    let prefixed = format!("{}_{}", target_triple_prefix(), name);
    env::var(&prefixed).ok().or_else(|| env::var(name).ok())
}

fn get_link_kind() -> LinkKind {
    if get_env("LIBCAMERA_STATIC").as_deref().is_some_and(|v| v == "1") {
        LinkKind::Static
    } else if get_env("LIBCAMERA_DYNAMIC").as_deref().is_some_and(|v| v == "1") {
        LinkKind::Dynamic
    } else {
        LinkKind::Default
    }
}

fn link_kind_cargo_str(kind: &LinkKind) -> &'static str {
    match kind {
        LinkKind::Static => "static=",
        LinkKind::Dynamic => "dylib=",
        LinkKind::Default => "",
    }
}

// ---------------------------------------------------------------------------
// Source selection
// ---------------------------------------------------------------------------

fn build_or_find_library(link_kind: LinkKind) -> Result<Library> {
    // Environment variable override
    if let Some(source) = get_env("LIBCAMERA_SOURCE") {
        return match source.as_str() {
            "vendor" => build_vendor(link_kind),
            "pkg-config" => find_pkg_config(link_kind),
            "explicit" => find_explicit(link_kind),
            other => bail!(
                "Unknown LIBCAMERA_SOURCE value '{}'. Expected: vendor, pkg-config, explicit",
                other
            ),
        };
    }

    // Feature-based selection: vendored first, then pkg-config, then explicit
    #[cfg(feature = "vendored")]
    return build_vendor(link_kind);

    #[cfg(all(not(feature = "vendored"), feature = "pkg-config"))]
    return find_pkg_config(link_kind);

    #[cfg(not(any(feature = "vendored", feature = "pkg-config")))]
    return find_explicit(link_kind);
}

// ---------------------------------------------------------------------------
// Vendored build via meson
// ---------------------------------------------------------------------------

/// Parse the target triple's architecture into meson's (cpu_family, cpu, endian).
fn parse_target_arch(target: &str) -> Result<(&'static str, &'static str, &'static str)> {
    let arch = target.split('-').next().unwrap_or("");
    match arch {
        "aarch64" => Ok(("aarch64", "aarch64", "little")),
        "armv7" => Ok(("arm", "armv7hl", "little")),
        "riscv64gc" => Ok(("riscv64", "riscv64", "little")),
        "x86_64" => Ok(("x86_64", "x86_64", "little")),
        "i686" | "i586" => Ok(("x86", "i686", "little")),
        "powerpc64le" => Ok(("ppc64", "ppc64le", "little")),
        "powerpc64" => Ok(("ppc64", "ppc64", "big")),
        "s390x" => Ok(("s390x", "s390x", "big")),
        _ => bail!("Unsupported target architecture '{}' for meson cross-file", arch),
    }
}

/// Generate a meson cross-file when cross-compiling (HOST != TARGET).
///
/// Reads cross-compiler paths from cc-rs convention env vars (e.g.
/// `CC_aarch64_unknown_linux_gnu`) and forwards `PKG_CONFIG_PATH` so meson
/// can find target-architecture dependencies like libyaml.
fn generate_meson_cross_file(out_dir: &Path) -> Result<Option<PathBuf>> {
    let host = env::var("HOST").unwrap_or_default();
    let target = env::var("TARGET").unwrap_or_default();

    if host == target {
        return Ok(None);
    }

    let target_under = target.replace('-', "_");

    // Read cross-compiler paths using cc-rs convention: CC_<target_underscored>
    let cc = env::var(format!("CC_{}", target_under))
        .or_else(|_| env::var("TARGET_CC"))
        .or_else(|_| env::var("CC"))
        .unwrap_or_else(|_| "cc".to_string());
    let cxx = env::var(format!("CXX_{}", target_under))
        .or_else(|_| env::var("TARGET_CXX"))
        .or_else(|_| env::var("CXX"))
        .unwrap_or_else(|_| "c++".to_string());
    let ar = env::var(format!("AR_{}", target_under))
        .or_else(|_| env::var("TARGET_AR"))
        .or_else(|_| env::var("AR"))
        .unwrap_or_else(|_| "ar".to_string());

    let (cpu_family, cpu, endian) = parse_target_arch(&target)?;
    let system = if target.contains("linux") {
        "linux"
    } else if target.contains("windows") {
        "windows"
    } else if target.contains("darwin") || target.contains("apple") {
        "darwin"
    } else {
        bail!("Unsupported OS in target triple: {}", target)
    };

    let mut content = format!(
        "[binaries]\n\
         c = '{}'\n\
         cpp = '{}'\n\
         ar = '{}'\n\
         pkg-config = 'pkg-config'\n",
        cc, cxx, ar
    );

    // Auto-detect lld for cross-linking. When the compiler wrappers don't
    // embed -fuse-ld=lld (to avoid meson's -Werror=unused-command-line-argument
    // during compile-only checks), we specify the linker explicitly here.
    // Meson only passes -fuse-ld=<value> during link steps, not compile checks.
    if Command::new("ld.lld")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        content.push_str("c_ld = 'lld'\ncpp_ld = 'lld'\n");
    }

    content.push_str(&format!(
        "\n[host_machine]\n\
         system = '{}'\n\
         cpu_family = '{}'\n\
         cpu = '{}'\n\
         endian = '{}'\n",
        system, cpu_family, cpu, endian
    ));

    // Forward PKG_CONFIG_PATH so meson can find cross-compiled dependencies
    let pkg_config_path = env::var("PKG_CONFIG_PATH").unwrap_or_default();
    if !pkg_config_path.is_empty() {
        let paths: Vec<String> = pkg_config_path
            .split(':')
            .filter(|p| !p.is_empty())
            .map(|p| format!("'{}'", p))
            .collect();
        content.push_str(&format!(
            "\n[built-in options]\npkg_config_path = [{}]\n",
            paths.join(", ")
        ));
    }

    let cross_file_path = out_dir.join("meson-cross.ini");
    fs::write(&cross_file_path, &content).context("Failed to write meson cross-file")?;

    eprintln!(
        "Generated meson cross-file for target '{}' at {}",
        target,
        cross_file_path.display()
    );

    Ok(Some(cross_file_path))
}

fn build_vendor(link_kind: LinkKind) -> Result<Library> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let source_path = manifest_dir.join("libcamera");
    if !source_path.join("meson.build").exists() {
        bail!(
            "Vendored libcamera source not found at {}. \
             Make sure git submodules are initialized: \
             `git submodule update --init --recursive`",
            source_path.display()
        );
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let build_path = out_dir.join("libcamera-build");
    let install_path = out_dir.join("libcamera-install");

    // meson setup
    let default_library = match link_kind {
        LinkKind::Static | LinkKind::Default => "static",
        LinkKind::Dynamic => "shared",
    };

    let mut setup_cmd = Command::new("meson");
    setup_cmd
        .arg("setup")
        .arg(&build_path)
        .arg(&source_path)
        .arg(format!("--prefix={}", install_path.display()))
        .arg("--buildtype=release")
        .arg(format!("--default-library={}", default_library))
        // Disable optional components not needed for Rust bindings
        .arg("-Dcam=disabled")
        .arg("-Dqcam=disabled")
        .arg("-Dgstreamer=disabled")
        .arg("-Dtracing=disabled")
        .arg("-Dpycamera=disabled")
        .arg("-Dlc-compliance=disabled")
        .arg("-Ddocumentation=disabled")
        .arg("-Dtest=false")
        .arg("-Dv4l2=disabled")
        // Prevent meson from downloading wrap dependencies during the build.
        // This avoids network access and ensures reproducible builds.
        .arg("--wrap-mode=nodownload");

    // Reconfigure if the build directory already exists from a previous run.
    if build_path.join("build.ninja").exists() {
        setup_cmd.arg("--reconfigure");
    }

    // On ARM targets, the rpi/pisp pipeline is auto-selected but requires
    // libpisp which is only available via meson wrap download. Exclude it
    // by explicitly listing all other ARM pipelines.
    // Users can override this with LIBCAMERA_PIPELINES env var.
    let target = env::var("TARGET").unwrap_or_default();
    if let Ok(pipelines) = env::var("LIBCAMERA_PIPELINES") {
        setup_cmd.arg(format!("-Dpipelines={}", pipelines));
    } else if target.starts_with("aarch64") || target.starts_with("arm") {
        setup_cmd.arg("-Dpipelines=imx8-isi,mali-c55,rkisp1,rpi/vc4,simple,uvcvideo");
    }

    // Add cross-file when cross-compiling (HOST != TARGET)
    if let Some(cross_file) = generate_meson_cross_file(&out_dir)? {
        setup_cmd.arg(format!("--cross-file={}", cross_file.display()));
    }

    run_command(&mut setup_cmd).context("Failed to run meson setup. Is meson installed?")?;

    // meson compile
    run_command(Command::new("meson").arg("compile").arg("-C").arg(&build_path))
        .context("Failed to compile libcamera with meson")?;

    // meson install
    run_command(Command::new("meson").arg("install").arg("-C").arg(&build_path))
        .context("Failed to install libcamera with meson")?;

    // Find the lib directory (could be lib, lib64, or lib/<triple>)
    let lib_path = find_lib_dir(&install_path);
    let version = read_meson_version(&source_path)?;

    // Emit cargo directives.
    // libcamera depends on libcamera-base; list camera first so GNU ld resolves
    // camera's references to camera-base symbols correctly (left-to-right order).
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    let link_prefix = link_kind_cargo_str(&link_kind);
    println!("cargo:rustc-link-lib={}camera", link_prefix);
    println!("cargo:rustc-link-lib={}camera-base", link_prefix);

    // Propagate to dependent crates via DEP_CAMERA_* env vars
    println!("cargo:VERSION={}", version);
    // Set INCLUDE to include/libcamera (matches pkg-config -I output structure)
    let include_base = install_path.join("include").join("libcamera");
    println!("cargo:INCLUDE={}", include_base.display());

    let include_libcamera = include_base.join("libcamera");

    Ok(Library {
        include_paths: vec![include_libcamera, include_base],
    })
}

fn run_command(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute {:?}", cmd.get_program()))?;
    if !status.success() {
        bail!("Command {:?} failed with exit code: {}", cmd.get_program(), status);
    }
    Ok(())
}

fn find_lib_dir(install_path: &Path) -> PathBuf {
    let lib_names = ["libcamera.a", "libcamera.so", "libcamera.dylib"];
    // Try common lib directory names
    for dir in &["lib", "lib64"] {
        let path = install_path.join(dir);
        if path.exists() {
            // Check for architecture-specific subdirectory (e.g. lib/x86_64-linux-gnu)
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() && lib_names.iter().any(|name| entry_path.join(name).exists()) {
                        return entry_path;
                    }
                }
            }
            // Check the lib dir itself
            if lib_names.iter().any(|name| path.join(name).exists()) {
                return path;
            }
        }
    }
    // Fallback
    install_path.join("lib")
}

fn read_meson_version(source_path: &Path) -> Result<String> {
    let meson_build = fs::read_to_string(source_path.join("meson.build")).context("Failed to read meson.build")?;

    // The version is in the project() call at the top of meson.build.
    // Limit to first 20 lines to avoid matching dependency versions deeper in the file.
    for line in meson_build.lines().take(20) {
        let line = line.trim();
        // Skip meson_version lines
        if line.starts_with("meson_version") {
            continue;
        }
        if line.contains("version") && line.contains(':') {
            // Look for version : 'X.Y.Z'
            if let Some(start) = line.find('\'') {
                if let Some(end) = line[start + 1..].find('\'') {
                    let version = &line[start + 1..start + 1 + end];
                    // Must start with a digit (excludes '>= 1.0.1' etc.)
                    if version.starts_with(|c: char| c.is_ascii_digit()) && version.contains('.') {
                        return Ok(version.to_string());
                    }
                }
            }
        }
    }

    bail!(
        "Could not parse version from {}",
        source_path.join("meson.build").display()
    )
}

// ---------------------------------------------------------------------------
// pkg-config
// ---------------------------------------------------------------------------

#[cfg(feature = "pkg-config")]
fn find_pkg_config(link_kind: LinkKind) -> Result<Library> {
    let probe = |name: &str| -> Result<pkg_config::Library, pkg_config::Error> {
        let mut config = pkg_config::Config::new();
        if matches!(link_kind, LinkKind::Static) {
            config.statik(true);
        }
        if matches!(link_kind, LinkKind::Dynamic) {
            // Suppress default cargo metadata so we can emit dylib= prefix manually.
            config.cargo_metadata(false);
        }
        config.probe(name)
    };

    let lib = probe("libcamera")
        .or_else(|e| {
            // Older libcamera versions use "camera" instead of "libcamera"
            probe("camera").map_err(|_| e)
        })
        .context("Failed to find libcamera via pkg-config")?;

    // For Dynamic, we suppressed pkg-config's cargo metadata above and emit manually.
    if matches!(link_kind, LinkKind::Dynamic) {
        for path in &lib.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for name in &lib.libs {
            println!("cargo:rustc-link-lib=dylib={}", name);
        }
    }
    // For Default and Static, pkg-config already emitted the correct directives.

    // Propagate to dependent crates via DEP_CAMERA_* env vars
    println!("cargo:VERSION={}", lib.version);
    if let Some(include_path) = lib.include_paths.first() {
        println!("cargo:INCLUDE={}", include_path.display());
    }

    Ok(Library {
        include_paths: lib.include_paths,
    })
}

#[cfg(not(feature = "pkg-config"))]
fn find_pkg_config(_link_kind: LinkKind) -> Result<Library> {
    bail!(
        "pkg-config feature is not enabled. \
         Enable the 'pkg-config' feature or use a different LIBCAMERA_SOURCE."
    );
}

// ---------------------------------------------------------------------------
// Explicit paths via env vars
// ---------------------------------------------------------------------------

fn find_explicit(link_kind: LinkKind) -> Result<Library> {
    let lib_dir =
        get_env("LIBCAMERA_LIB_DIR").context("LIBCAMERA_LIB_DIR must be set when using explicit source mode")?;
    let include_dir = get_env("LIBCAMERA_INCLUDE_DIR")
        .context("LIBCAMERA_INCLUDE_DIR must be set when using explicit source mode")?;

    let lib_path = PathBuf::from(&lib_dir);
    let include_path = PathBuf::from(&include_dir);

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib={}camera", link_kind_cargo_str(&link_kind));

    // Propagate to dependent crates
    // Try to detect version from the include path's headers
    let version = detect_version_from_includes(&include_path).unwrap_or_default();
    if !version.is_empty() {
        println!("cargo:VERSION={}", version);
    }
    println!("cargo:INCLUDE={}", include_path.display());

    let include_libcamera = include_path.join("libcamera");
    let mut include_paths = vec![];
    if include_libcamera.exists() {
        include_paths.push(include_libcamera);
    }
    include_paths.push(include_path);

    Ok(Library { include_paths })
}

fn detect_version_from_includes(include_path: &Path) -> Option<String> {
    // Try to find version.h
    let version_h = include_path.join("libcamera").join("version.h");
    if let Ok(contents) = fs::read_to_string(&version_h) {
        for line in contents.lines() {
            if line.contains("LIBCAMERA_VERSION_STRING") {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        return Some(line[start + 1..start + 1 + end].to_string());
                    }
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Compile C API wrapper
// ---------------------------------------------------------------------------

fn compile_c_api(library: &Library) -> Result<()> {
    let mut c_api_headers: Vec<PathBuf> = Vec::new();
    let mut cpp_api_headers: Vec<PathBuf> = Vec::new();
    let mut c_api_sources: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir("c_api").context("Failed to read c_api directory")? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
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

    let mut build = cc::Build::new();
    build.cpp(true).flag("-std=c++17").files(&c_api_sources);

    for path in &library.include_paths {
        build.include(path);
        // Also add parent of the include path (e.g. if path is .../libcamera, add ...)
        if let Some(parent) = path.parent() {
            build.include(parent);
        }
    }

    build.compile("camera_c_api");
    Ok(())
}

// ---------------------------------------------------------------------------
// Generate bindgen bindings
// ---------------------------------------------------------------------------

fn generate_bindings(library: &Library) -> Result<()> {
    #[cfg(feature = "pregenerated-bindings")]
    {
        // Use pre-generated bindings when the feature is enabled.
        // This avoids requiring libclang at build time, which is necessary for
        // cross-compilation on musl targets where the build script cannot
        // dlopen libclang.so (musl static binaries don't support dlopen).
        let out_path = PathBuf::from(env::var("OUT_DIR")?);
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR not set")?);
        let bindings_dir = manifest_dir.join("bindings");
        fs::copy(bindings_dir.join("bindings.rs"), out_path.join("bindings.rs"))
            .context("Failed to copy pre-generated C bindings")?;
        fs::copy(bindings_dir.join("bindings_cpp.rs"), out_path.join("bindings_cpp.rs"))
            .context("Failed to copy pre-generated C++ bindings")?;
        let _ = library;
        return Ok(());
    }

    #[cfg(all(not(feature = "pregenerated-bindings"), feature = "bindgen"))]
    {
        let out_path = PathBuf::from(env::var("OUT_DIR")?);
        return generate_bindings_with_bindgen(library, &out_path);
    }

    #[cfg(not(any(feature = "pregenerated-bindings", feature = "bindgen")))]
    {
        let _ = library;
        bail!(
            "Either the 'bindgen' or 'pregenerated-bindings' feature must be enabled. \
             Add 'bindgen' to default features or enable 'pregenerated-bindings'."
        );
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings_with_bindgen(library: &Library, out_path: &Path) -> Result<()> {
    let mut c_api_headers: Vec<PathBuf> = Vec::new();
    let mut cpp_api_headers: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir("c_api").context("Failed to read c_api directory")? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        match entry.path().extension().and_then(|s| s.to_str()) {
            Some("h") => c_api_headers.push(entry.path()),
            Some("hpp") => cpp_api_headers.push(entry.path()),
            _ => {}
        }
    }

    // C bindings
    let mut builder = bindgen::Builder::default()
        .constified_enum_module("libcamera_.*")
        .allowlist_function("libcamera_.*")
        .allowlist_var("LIBCAMERA_.*")
        .allowlist_var(".*LIBCAMERA_VERSION.*")
        .allowlist_type("libcamera_.*");

    for path in &library.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    for header in &c_api_headers {
        builder = builder.header(header.to_str().unwrap());
    }

    let bindings = builder.generate().context("Unable to generate C bindings")?;
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .context("Couldn't write C bindings")?;

    // CPP bindings
    let mut builder = bindgen::Builder::default()
        .clang_arg("-std=c++17")
        .allowlist_type(".*controls.*")
        .allowlist_type(".*properties.*");

    for path in &library.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    for header in &cpp_api_headers {
        builder = builder.header(header.to_str().unwrap());
    }

    let bindings = builder.generate().context("Unable to generate CPP bindings")?;
    bindings
        .write_to_file(out_path.join("bindings_cpp.rs"))
        .context("Couldn't write CPP bindings")?;

    Ok(())
}
