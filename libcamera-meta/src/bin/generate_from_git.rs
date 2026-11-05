use std::{
    collections::{BTreeMap, HashMap},
    fmt::Write,
    path::Path,
};

use git2::{build::CheckoutBuilder, ObjectType, Repository};
use libcamera_meta::{ControlEnumValue, ControlSize, ControlType};
use regex::Regex;
use semver::Version;
use yaml_rust::{Yaml, YamlLoader};

use crate::generate_rust::ControlsType;

struct ByVersionData {
    pub controls: BTreeMap<String, String>,
    pub properties: BTreeMap<String, String>,
    pub formats_yaml: String,
    pub formats_cpp: String,
}

#[derive(Debug)]
pub struct Control {
    pub name: String,
    pub vendor: String,

    pub typ: ControlType,
    pub description: String,
    pub size: Option<Vec<ControlSize>>,
    pub enumeration: Option<Vec<ControlEnumValue>>,
}

#[derive(Debug, Clone)]
struct FormatConst {
    name: String,
    fourcc: u32,
    modifier: u64,
}

#[derive(Debug, Clone)]
struct PixelFormatInfo {
    name: String,
    fourcc: u32,
    modifier: u64,
    bits_per_pixel: u32,
    colour_encoding: u8,
    packed: bool,
    pixels_per_group: u32,
    planes: Vec<(u32, u32)>,
    v4l2_formats: Vec<u32>,
}

fn main() {
    let versioned_files = Path::new("libcamera/versioned_files");
    let _ = std::fs::remove_dir_all(versioned_files);
    let _ = std::fs::create_dir_all(versioned_files);

    let git_dir = Path::new("libcamera-git");

    let repo = Repository::open(git_dir).unwrap_or_else(|_| {
        Repository::clone("https://git.libcamera.org/libcamera/libcamera.git", git_dir)
            .expect("Failed to clone libcamera")
    });

    if let Some(mut remote) = repo.remotes().ok().and_then(|remote_names| {
        remote_names
            .iter()
            .flatten()
            .filter_map(|name| repo.find_remote(name).ok())
            .next()
    }) {
        let mut options = git2::FetchOptions::new();
        options.download_tags(git2::AutotagOption::All);

        remote
            .fetch(&["master"], Some(&mut options), None)
            .expect("failed to fetch libcamera");
    }

    let mut by_version = BTreeMap::<Version, ByVersionData>::new();

    repo.tag_foreach(|id, name| {
        let name = std::str::from_utf8(name).unwrap();
        println!("Found tag {name}");

        let version = name.split('/').next_back().unwrap();
        if !version.starts_with('v') {
            return true;
        }
        let version = match Version::parse(&version[1..]) {
            Ok(v) => v,
            Err(_) => {
                return true;
            }
        };

        if version == Version::new(0, 0, 0) {
            // Version 0.0.0 is just an empty repo
            return true;
        }

        if version.major == 0 && version.minor < 4 {
            // Versions bellow v0.4.0 are incompatible with newer control values
            println!("Skipping unsupported version {version}");
            return true;
        }

        println!("Extracting files for version {version}");

        let object = repo.find_object(id, Some(ObjectType::Tag)).unwrap();

        repo.checkout_tree(&object, Some(CheckoutBuilder::new().force()))
            .unwrap();

        let extract_controls = |filter_prefix: &str| -> BTreeMap<String, String> {
            std::fs::read_dir(git_dir.join("src/libcamera"))
                .unwrap()
                .filter_map(|entry| {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path
                        .file_name()
                        .map(|name| name.to_string_lossy().starts_with(filter_prefix))
                        .unwrap_or(false)
                        && path
                            .extension()
                            .map(|ext| ext.to_string_lossy() == "yaml")
                            .unwrap_or(false)
                    {
                        Some(path)
                    } else {
                        None
                    }
                })
                .map(|control_path| {
                    let name = control_path.file_name().unwrap().to_string_lossy().to_string();
                    let contents = std::fs::read_to_string(control_path.as_path()).unwrap();
                    (name, contents)
                })
                .collect()
        };
        let controls = extract_controls("control_ids");
        let properties = extract_controls("property_ids");
        let formats_yaml =
            std::fs::read_to_string(git_dir.join("src/libcamera/formats.yaml")).expect("read formats.yaml");
        let formats_cpp = std::fs::read_to_string(git_dir.join("src/libcamera/formats.cpp")).expect("read formats.cpp");

        by_version.insert(
            version,
            ByVersionData {
                controls,
                properties,
                formats_yaml,
                formats_cpp,
            },
        );

        true
    })
    .unwrap();

    println!("\n\n");

    fn parse_control_files(files: &BTreeMap<String, String>) -> Vec<Control> {
        let control_yamls = files
            .values()
            .flat_map(|contents| YamlLoader::load_from_str(contents).unwrap());

        let mut controls = Vec::new();

        for root in control_yamls {
            let vendor = root
                .as_hash()
                .unwrap()
                .get(&Yaml::String("vendor".to_string()))
                .and_then(|v| v.as_str());

            for (control_name, val) in root["controls"]
                .as_vec()
                .unwrap()
                .iter()
                .flat_map(|control| control.as_hash().unwrap().iter())
            {
                let name = control_name.as_str().unwrap().to_string();

                let vendor = vendor.unwrap_or_else(|| {
                    if val
                        .as_hash()
                        .unwrap()
                        .get(&Yaml::String("draft".to_string()))
                        .and_then(|yml| yml.as_bool())
                        .unwrap_or(false)
                    {
                        "draft"
                    } else {
                        "libcamera"
                    }
                });

                let typ = val["type"].as_str().unwrap().try_into().unwrap();
                let description = val["description"].as_str().unwrap().to_string();
                let size = val["size"]
                    .as_vec()
                    .map(|s| s.iter().map(|s| s.try_into().unwrap()).collect());
                let enumeration = val["enum"].as_vec().map(|e| {
                    e.iter()
                        .map(|hash| ControlEnumValue {
                            name: hash["name"].as_str().unwrap().to_string(),
                            value: hash["value"].as_i64().unwrap() as _,
                            description: hash["description"].as_str().unwrap().to_string(),
                        })
                        .collect()
                });

                let control = Control {
                    name,
                    vendor: vendor.to_string(),
                    typ,
                    description,
                    size,
                    enumeration,
                };
                controls.push(control);
            }
        }

        controls
    }

    fn parse_format_consts(
        formats_yaml: &str,
        drm_map: &HashMap<String, u32>,
        modifier_map: &HashMap<String, u64>,
        v4l2_map: &HashMap<String, u32>,
    ) -> Vec<FormatConst> {
        let yaml = YamlLoader::load_from_str(formats_yaml).expect("parse formats.yaml");
        let mut out = Vec::new();
        for doc in yaml {
            let Some(formats) = doc["formats"].as_vec() else {
                continue;
            };
            for entry in formats {
                let map = entry.as_hash().unwrap();
                for (name_yaml, val) in map {
                    let name = name_yaml.as_str().unwrap().to_string();
                    let fourcc_name = val["fourcc"].as_str().unwrap();
                    let mut fourcc = drm_map
                        .get(fourcc_name)
                        .copied()
                        .or_else(|| {
                            let suffix = fourcc_name.trim_start_matches("DRM_FORMAT_");
                            let v4l2_name = format!("V4L2_PIX_FMT_{suffix}");
                            v4l2_map.get(&v4l2_name).copied()
                        })
                        .unwrap_or_else(|| {
                            let suffix = fourcc_name.trim_start_matches("DRM_FORMAT_");
                            let v4l2_name = format!("V4L2_PIX_FMT_{suffix}");
                            panic!("missing DRM fourcc for {fourcc_name} (also tried {v4l2_name})")
                        });
                    if val["big_endian"].as_bool().unwrap_or(false) {
                        fourcc |= 1u32 << 31;
                    }
                    let modifier = val["modifier"]
                        .as_str()
                        .and_then(|m| modifier_map.get(m).copied())
                        .unwrap_or(0);
                    out.push(FormatConst { name, fourcc, modifier });
                }
            }
        }
        out
    }

    fn build_drm_fourcc_map() -> HashMap<String, u32> {
        let drm_path = "/usr/include/drm/drm_fourcc.h";
        match std::fs::read_to_string(drm_path) {
            Ok(header) => {
                let drm_re =
                    Regex::new(r"#define\s+(DRM_FORMAT_[A-Za-z0-9_]+)\s+fourcc_code(_be)?\(([^)]+)\)").unwrap();
                let mut map = HashMap::new();
                for caps in drm_re.captures_iter(&header) {
                    let name = caps.get(1).unwrap().as_str().to_string();
                    let be = caps.get(2).is_some();
                    let args = caps.get(3).unwrap().as_str();
                    let parts: Vec<u32> = args
                        .split(',')
                        .filter_map(|p| p.trim().trim_matches('\'').chars().next().map(|c| c as u32))
                        .collect();
                    if parts.len() != 4 {
                        continue;
                    }
                    let mut fourcc = parts[0] | (parts[1] << 8) | (parts[2] << 16) | (parts[3] << 24);
                    if be {
                        fourcc |= 1u32 << 31;
                    }
                    map.insert(name, fourcc);
                }
                map
            }
            Err(err) => {
                eprintln!("Warning: failed to read {drm_path}: {err}");
                HashMap::new()
            }
        }
    }

    fn build_drm_modifier_map() -> HashMap<String, u64> {
        let drm_path = "/usr/include/drm/drm_fourcc.h";
        let mut map = HashMap::new();
        let Ok(header) = std::fs::read_to_string(drm_path) else {
            eprintln!("Warning: failed to read {drm_path} for modifiers");
            return map;
        };
        const VENDOR_RE: &str = r"#define\s+DRM_FORMAT_MOD_VENDOR_([A-Za-z0-9_]+)\s+([0-9xXa-fA-F]+)";
        const MOD_RE: &str = concat!(
            r"#define\s+DRM_FORMAT_MOD_([A-Za-z0-9_]+)\s+fourcc_mod_code\(",
            r"\s*([A-Za-z0-9_]+)\s*,\s*([0-9xXa-fA-F]+)\s*\)",
        );
        let vendor_re = Regex::new(VENDOR_RE).unwrap();
        let mod_re = Regex::new(MOD_RE).unwrap();
        let mut vendors = HashMap::new();
        for caps in vendor_re.captures_iter(&header) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let val_str = caps.get(2).unwrap().as_str();
            let val = u64::from_str_radix(val_str.trim_start_matches("0x"), 16).unwrap_or(0);
            vendors.insert(name, val);
        }
        for caps in mod_re.captures_iter(&header) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let vendor = caps.get(2).unwrap().as_str();
            let val_str = caps.get(3).unwrap().as_str();
            let val = u64::from_str_radix(val_str.trim_start_matches("0x"), 16).unwrap_or(0);
            let vendor_val = vendors.get(vendor).copied().unwrap_or(0);
            let modifier = (vendor_val << 56) | val;
            map.insert(format!("DRM_FORMAT_MOD_{name}"), modifier);
        }
        map
    }

    fn build_v4l2_fourcc_map() -> HashMap<String, u32> {
        let videodev_path = "/usr/include/linux/videodev2.h";
        match std::fs::read_to_string(videodev_path) {
            Ok(header) => {
                let v4l2_re =
                    Regex::new(r"#define\s+(V4L2_PIX_FMT_[A-Za-z0-9_]+)\s+v4l2_fourcc(_be)?\(([^)]+)\)").unwrap();
                let mut map = HashMap::new();
                for caps in v4l2_re.captures_iter(&header) {
                    let name = caps.get(1).unwrap().as_str().to_string();
                    let be = caps.get(2).is_some();
                    let args = caps.get(3).unwrap().as_str();
                    let parts: Vec<u32> = args
                        .split(',')
                        .filter_map(|p| p.trim().trim_matches('\'').chars().next().map(|c| c as u32))
                        .collect();
                    if parts.len() != 4 {
                        continue;
                    }
                    let mut fourcc = parts[0] | (parts[1] << 8) | (parts[2] << 16) | (parts[3] << 24);
                    if be {
                        fourcc |= 1u32 << 31;
                    }
                    map.insert(name, fourcc);
                }
                map
            }
            Err(err) => {
                eprintln!("Warning: failed to read {videodev_path}: {err}");
                HashMap::new()
            }
        }
    }

    fn parse_pixel_format_info(
        formats_cpp: &str,
        format_consts: &[FormatConst],
        v4l2_map: &HashMap<String, u32>,
    ) -> Vec<PixelFormatInfo> {
        let entry_re =
            Regex::new(r"(?s)\{\s*formats::(?P<name>\w+),\s*\{\s*(?P<body>.*?)\}\s*\}\s*,").expect("regex compile");
        let bits_re = Regex::new(r"\.bitsPerPixel\s*=\s*([0-9]+)").unwrap();
        let colour_re = Regex::new(r"ColourEncoding([A-Za-z]+)").unwrap();
        let packed_re = Regex::new(r"\.packed\s*=\s*(true|false)").unwrap();
        let ppg_re = Regex::new(r"\.pixelsPerGroup\s*=\s*([0-9]+)").unwrap();
        const PLANES_RE: &str = concat!(
            r"\.planes\s*=\s*\{\{\s*\{\s*([0-9]+)\s*,\s*([0-9]+)\s*\}\s*,",
            r"\s*\{\s*([0-9]+)\s*,\s*([0-9]+)\s*\}\s*,",
            r"\s*\{\s*([0-9]+)\s*,\s*([0-9]+)\s*\}\s*\}\s*\}",
        );
        let planes_re = Regex::new(PLANES_RE).unwrap();
        let v4l2_list_re = Regex::new(r"\.v4l2Formats\s*=\s*\{(?P<formats>[^}]*)\}").unwrap();
        let v4l2_item_re = Regex::new(r"V4L2PixelFormat\(\s*(V4L2_PIX_FMT_[A-Za-z0-9_]+)\s*\)").unwrap();

        let const_map: HashMap<_, _> = format_consts
            .iter()
            .map(|c| (c.name.clone(), (c.fourcc, c.modifier)))
            .collect();

        let mut entries = Vec::new();
        for caps in entry_re.captures_iter(formats_cpp) {
            let name = caps["name"].to_string();
            let body = &caps["body"];
            let (fourcc, modifier) = match const_map.get(&name) {
                Some(v) => *v,
                None => continue,
            };
            let bits_per_pixel = bits_re
                .captures(body)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(0);
            let colour_encoding = colour_re
                .captures(body)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .unwrap_or("RGB");
            let colour_encoding = match colour_encoding {
                "RGB" => 0u8,
                "YUV" => 1u8,
                "RAW" => 2u8,
                _ => 0u8,
            };
            let packed = packed_re
                .captures(body)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str() == "true")
                .unwrap_or(false);
            let pixels_per_group = ppg_re
                .captures(body)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(1);
            let planes_caps = planes_re.captures(body);
            let plane_vals: Vec<(u32, u32)> = planes_caps
                .map(|p| {
                    (1..=6)
                        .filter_map(|idx| p.get(idx).and_then(|m| m.as_str().parse::<u32>().ok()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_else(|| vec![0; 6])
                .chunks(2)
                .map(|c| (c.first().copied().unwrap_or(0), c.get(1).copied().unwrap_or(0)))
                .collect();

            let mut v4l2_formats: Vec<u32> = Vec::new();
            if let Some(list_caps) = v4l2_list_re.captures(body) {
                if let Some(list) = list_caps.name("formats") {
                    for item in v4l2_item_re.captures_iter(list.as_str()) {
                        let name = item.get(1).unwrap().as_str();
                        if let Some(val) = v4l2_map.get(name) {
                            v4l2_formats.push(*val);
                        }
                    }
                }
            }

            entries.push(PixelFormatInfo {
                name,
                fourcc,
                modifier,
                bits_per_pixel,
                colour_encoding,
                packed,
                pixels_per_group,
                planes: plane_vals,
                v4l2_formats,
            });
        }

        entries
    }

    fn generate_pixel_format_info_rs(infos: &[PixelFormatInfo]) -> String {
        let mut out = String::from(
            r#"
// Auto-generated from libcamera/src/libcamera/formats.cpp
#[derive(Clone, Copy)]
pub(crate) struct PixelFormatPlaneInfoData {
    pub bytes_per_group: u32,
    pub vertical_sub_sampling: u32,
}

#[derive(Clone, Copy)]
pub(crate) struct PixelFormatInfoData {
    pub name: &'static str,
    pub fourcc: u32,
    pub modifier: u64,
    pub bits_per_pixel: u32,
    pub colour_encoding: u8,
    pub packed: bool,
    pub pixels_per_group: u32,
    pub planes: &'static [PixelFormatPlaneInfoData],
    pub v4l2_formats: &'static [u32],
}

pub(crate) static PIXEL_FORMAT_INFO: &[PixelFormatInfoData] = &[
"#,
        );

        for info in infos {
            let planes: Vec<(u32, u32)> = (0..3)
                .map(|idx| info.planes.get(idx).copied().unwrap_or((0, 0)))
                .collect();
            let v4l2_list = info
                .v4l2_formats
                .iter()
                .map(|v| format!("0x{v:08x}"))
                .collect::<Vec<_>>()
                .join(", ");

            let _ = writeln!(
                out,
                "    PixelFormatInfoData {{ \
                 name: \"{}\", fourcc: 0x{:08x}, modifier: 0x{:016x}, bits_per_pixel: {}, \
                 colour_encoding: {}, packed: {}, pixels_per_group: {}, \
                 planes: &[PixelFormatPlaneInfoData {{ bytes_per_group: {}, vertical_sub_sampling: {} }}, \
                 PixelFormatPlaneInfoData {{ bytes_per_group: {}, vertical_sub_sampling: {} }}, \
                 PixelFormatPlaneInfoData {{ bytes_per_group: {}, vertical_sub_sampling: {} }}], \
                 v4l2_formats: &[{}], \
                 }},",
                info.name,
                info.fourcc,
                info.modifier,
                info.bits_per_pixel,
                info.colour_encoding,
                info.packed,
                info.pixels_per_group,
                planes[0].0,
                planes[0].1,
                planes[1].0,
                planes[1].1,
                planes[2].0,
                planes[2].1,
                v4l2_list,
            );
        }

        out.push_str("];\n");
        out
    }

    let drm_map = build_drm_fourcc_map();
    let drm_modifier_map = build_drm_modifier_map();
    let v4l2_map = build_v4l2_fourcc_map();

    for (version, data) in by_version.iter() {
        let output_dir = versioned_files.join(version.to_string());
        std::fs::create_dir_all(output_dir.as_path()).unwrap();

        for (name, contents) in data.controls.iter().chain(data.properties.iter()) {
            std::fs::write(output_dir.join(name), contents).unwrap();
        }

        println!("Parsing controls for version {version}");
        let controls = parse_control_files(&data.controls);
        std::fs::write(
            output_dir.join("controls.rs"),
            generate_rust::generate_controls_file(&controls, ControlsType::Control),
        )
        .unwrap();

        println!("Parsing properties for version {version}");
        let properties = parse_control_files(&data.properties);
        std::fs::write(
            output_dir.join("properties.rs"),
            generate_rust::generate_controls_file(&properties, ControlsType::Property),
        )
        .unwrap();

        println!("Parsing pixel formats for version {version}");
        let format_consts = parse_format_consts(&data.formats_yaml, &drm_map, &drm_modifier_map, &v4l2_map);
        let pf_info = parse_pixel_format_info(&data.formats_cpp, &format_consts, &v4l2_map);
        std::fs::write(
            output_dir.join("pixel_format_info.rs"),
            generate_pixel_format_info_rs(&pf_info),
        )
        .unwrap();
    }
}

mod generate_rust {
    use libcamera_meta::{ControlSize, ControlType};

    use crate::{to_c_type_name, Control};

    fn format_docstring(desc: &str, indent: usize) -> String {
        let mut out = String::new();
        let mut in_text_block = false;

        for line in desc.trim().split('\n') {
            if !in_text_block && line.starts_with("  ") {
                in_text_block = true;
                out.push_str(&format!("{}/// ```text\n", " ".repeat(indent)))
            } else if in_text_block && !line.starts_with("  ") {
                in_text_block = false;
                out.push_str(&format!("{}/// ```\n", " ".repeat(indent)))
            }

            out.push_str(&format!("{}/// {}\n", " ".repeat(indent), line))
        }

        out
    }

    fn to_rust_type(t: ControlType, size: &Option<Vec<ControlSize>>) -> String {
        let inner = match t {
            ControlType::Bool => "bool",
            ControlType::Byte => "u8",
            ControlType::Uint16 => "u16",
            ControlType::Uint32 => "u32",
            ControlType::Int32 => "i32",
            ControlType::Int64 => "i64",
            ControlType::Float => "f32",
            ControlType::String => "String",
            ControlType::Rectangle => "Rectangle",
            ControlType::Size => "Size",
            ControlType::Point => "Point",
        };

        match size {
            Some(s) => {
                if s.is_empty() {
                    panic!("Array-like datatype with zero dimensions");
                } else if matches!(s[0], ControlSize::Dynamic) {
                    if s.len() > 1 {
                        panic!("Dynamic length with more than 1 dimension is not supported");
                    } else {
                        format!("Vec<{inner}>")
                    }
                } else {
                    s.iter().fold(inner.to_string(), |ty, s| match s {
                        ControlSize::Dynamic => panic!("Dynamic length with more than 1 dimension is not supported"),
                        ControlSize::Fixed(len) => format!("[{ty}; {len}]"),
                    })
                }
            }
            None => inner.to_string(),
        }
    }

    pub enum ControlsType {
        Control,
        Property,
    }

    fn generate_controls(controls: &[Control], ty: ControlsType) -> String {
        fn vendor_feature_gate(control: &Control) -> String {
            if control.vendor != "libcamera" {
                format!(r#"#[cfg(feature="vendor_{}")]"#, control.vendor)
            } else {
                "".to_string()
            }
        }

        let mut out = String::new();

        let name = match ty {
            ControlsType::Control => "ControlId",
            ControlsType::Property => "PropertyId",
        };

        out += "#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]\n";
        out += "#[repr(u32)]\n";
        out += &format!("pub enum {name} {{\n");
        for ctrl in controls.iter() {
            out += &format_docstring(&ctrl.description, 4);
            out += &format!(
                "    {}{} = {},\n",
                vendor_feature_gate(ctrl),
                ctrl.name,
                to_c_type_name(&ctrl.name).to_ascii_uppercase()
            );
        }
        out += "}\n";

        out += &format!("impl {name} {{\n");
        out += r#"
            pub fn id(&self) -> u32 {
                u32::from(*self)
            }
            "#;
        out += "    pub fn description(&self) -> &'static str {\n        match self {\n";
        for ctrl in controls.iter() {
            let desc = ctrl.description.replace('\\', "\\\\").replace('"', "\\\"");
            out += &vendor_feature_gate(ctrl);
            out += &format!("            {name}::{} => \"{}\",\n", ctrl.name, desc);
        }
        out += "        }\n    }\n";
        out += "}\n";

        let mut dyn_variants = String::new();

        for ctrl in controls.iter() {
            let ctrl_name = &ctrl.name;
            let ctrl_type = to_rust_type(ctrl.typ, &ctrl.size);

            out += &format_docstring(&ctrl.description, 0);
            if let Some(enumeration) = &ctrl.enumeration {
                out += &vendor_feature_gate(ctrl);
                out += "#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]";
                out += &format!("#[repr({ctrl_type})]");
                out += &format!("pub enum {ctrl_name} {{");
                for val in enumeration {
                    let var_name = val.name.replace(&ctrl.name, "");

                    out += &format_docstring(&val.description, 4);
                    out += &format!("    {var_name} = {},\n", val.value);
                }
                out += "}\n";

                out += &format!(
                    r#"
                    {0}
                impl TryFrom<ControlValue> for {ctrl_name} {{
                    type Error = ControlValueError;

                    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {{
                        Self::try_from({ctrl_type}::try_from(value.clone())?)
                            .map_err(|_| ControlValueError::UnknownVariant(value))
                    }}
                }}

                {0}
                impl From<{ctrl_name}> for ControlValue {{
                    fn from(val: {ctrl_name}) -> Self {{
                        ControlValue::from(<{ctrl_type}>::from(val))
                    }}
                }}
            "#,
                    vendor_feature_gate(ctrl)
                );
            } else {
                out += &format!(
                    r#"
                {0}
                #[derive(Debug, Clone)]
                pub struct {ctrl_name}(pub {ctrl_type});

                {0}
                impl Deref for {ctrl_name} {{
                    type Target = {ctrl_type};

                    fn deref(&self) -> &Self::Target {{
                        &self.0
                    }}
                }}

                {0}
                impl DerefMut for {ctrl_name} {{
                    fn deref_mut(&mut self) -> &mut Self::Target {{
                        &mut self.0
                    }}
                }}

                {0}
                impl TryFrom<ControlValue> for {ctrl_name} {{
                    type Error = ControlValueError;

                    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {{
                        Ok(Self(<{ctrl_type}>::try_from(value)?))
                    }}
                }}

                {0}
                impl From<{ctrl_name}> for ControlValue {{
                    fn from(val: {ctrl_name}) -> Self {{
                        ControlValue::from(val.0)
                    }}
                }}
            "#,
                    vendor_feature_gate(ctrl)
                );
            }

            out += &format!(
                r#"
            {0}
            impl ControlEntry for {ctrl_name} {{
                const ID: u32 = {name}::{ctrl_name} as _;
            }}
            "#,
                vendor_feature_gate(ctrl)
            );

            out += &vendor_feature_gate(ctrl);
            out += &match ty {
                ControlsType::Control => format!("impl Control for {ctrl_name} {{}}\n"),
                ControlsType::Property => format!("impl Property for {ctrl_name} {{}}\n"),
            };

            dyn_variants.push_str(&format!(
                "{0} {name}::{ctrl_name} => Ok(Box::new({ctrl_name}::try_from(val)?)),\n",
                vendor_feature_gate(ctrl),
            ));
        }

        out += &format!(
            r#"
        pub fn make_dyn(id: {name}, val: ControlValue) -> Result<Box<dyn DynControlEntry>, ControlValueError> {{
            match id {{
                {dyn_variants}
            }}
        }}
    "#
        );

        out
    }

    pub fn generate_controls_file(controls: &[Control], ty: ControlsType) -> String {
        let header = r#"
                use std::ops::{{Deref, DerefMut}};
                use num_enum::{{IntoPrimitive, TryFromPrimitive}};
                #[allow(unused_imports)]
                use crate::control::{{Control, Property, ControlEntry, DynControlEntry}};
                use crate::control_value::{{ControlValue, ControlValueError}};
                #[allow(unused_imports)]
                use crate::geometry::{{Rectangle, Point, Size}};
                #[allow(unused_imports)]
                use libcamera_sys::*;
                "#;

        let file = format!("{header}\n{}", generate_controls(controls, ty));
        prettyplease::unparse(&syn::parse_file(&file).unwrap())
    }
}

pub fn to_c_type_name(str: &str) -> String {
    let mut out = String::new();
    let chars = str.chars().collect::<Vec<_>>();

    for i in 0..chars.len() {
        // Do not split first char
        if i > 0 {
            let mut split = false;

            // Split if character is uppercase and previous char is lowercase
            if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
                split = true;
            }

            // Split if character is uppercase and following char is lowercase
            if chars[i].is_uppercase() && chars.get(i + 1).copied().map(char::is_lowercase).unwrap_or(false) {
                split = true;
            }

            // Split if previous character is numeric and current is not
            if !chars[i].is_numeric() && chars[i - 1].is_numeric() {
                split = true;
            }

            if split {
                out.push('_');
            }
        }

        out.push(chars[i].to_ascii_lowercase());
    }

    out
}
