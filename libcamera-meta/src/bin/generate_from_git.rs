use std::{collections::BTreeMap, path::Path};

use git2::{build::CheckoutBuilder, ObjectType, Repository};
use libcamera_meta::{ControlEnumValue, ControlSize, ControlType};
use semver::Version;
use yaml_rust::{Yaml, YamlLoader};

use crate::generate_rust::ControlsType;

struct ByVersionData {
    pub controls: BTreeMap<String, String>,
    pub properties: BTreeMap<String, String>,
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

        let version = name.split('/').last().unwrap();
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

        by_version.insert(version, ByVersionData { controls, properties });

        true
    })
    .unwrap();

    println!("\n\n");

    fn parse_control_files(files: &BTreeMap<String, String>) -> Vec<Control> {
        let control_yamls = files
            .iter()
            .flat_map(|(_, contents)| YamlLoader::load_from_str(contents).unwrap());

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
        out += &format!("pub enum {} {{\n", name);
        for ctrl in controls.iter() {
            out += &format_docstring(&ctrl.description, 4);
            out += &format!(
                "    {}{} = {},\n",
                vendor_feature_gate(ctrl),
                &ctrl.name,
                to_c_type_name(&ctrl.name).to_ascii_uppercase()
            );
        }
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
                use crate::geometry::{{Rectangle, Size}};
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
