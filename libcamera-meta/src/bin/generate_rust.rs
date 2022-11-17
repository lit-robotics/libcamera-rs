use indoc::printdoc;
use libcamera_meta::{control_ids, property_ids, Control, ControlSize, ControlType};

fn format_docstring(desc: &str, indent: usize) -> String {
    let mut out = String::new();
    let mut in_text_block = false;

    for line in desc.trim().split("\n") {
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

enum ControlsType {
    Control,
    Property,
}

fn generate_controls(controls: &Vec<Control>, ty: ControlsType) {
    let name = match ty {
        ControlsType::Control => "ControlId",
        ControlsType::Property => "PropertyId",
    };

    let mut i = 1;
    println!("#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]");
    println!("#[repr(u32)]");
    println!("pub enum {} {{", name);
    for ctrl in controls.iter() {
        print!("{}", format_docstring(&ctrl.description, 4));
        println!("    {} = {},", &ctrl.name, i);
        i += 1;
    }
    println!("}}\n");

    let mut dyn_variants = String::new();

    for ctrl in controls.iter() {
        let ctrl_name = &ctrl.name;
        let ctrl_type = to_rust_type(ctrl.typ, &ctrl.size);

        print!("{}", format_docstring(&ctrl.description, 0));
        if let Some(enumeration) = &ctrl.enumeration {
            println!("#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]");
            println!("#[repr({})]", ctrl_type);
            println!("pub enum {ctrl_name} {{");
            for val in enumeration {
                let var_name = val.name.replace(&ctrl.name, "");

                print!("{}", format_docstring(&val.description, 4));
                println!("    {var_name} = {},", val.value);
            }
            println!("}}\n");

            printdoc! {"
                impl TryFrom<ControlValue> for {ctrl_name} {{
                    type Error = ControlValueError;
                
                    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {{
                        Self::try_from({ctrl_type}::try_from(value.clone())?).map_err(|_| ControlValueError::UnknownVariant(value))
                    }}
                }}
                
                impl From<{ctrl_name}> for ControlValue {{
                    fn from(val: {ctrl_name}) -> Self {{
                        ControlValue::from(<{ctrl_type}>::from(val))
                    }}
                }}
            "};
        } else {
            printdoc! {"
                #[derive(Debug, Clone)]
                pub struct {ctrl_name}(pub {ctrl_type});

                impl Deref for {ctrl_name} {{
                    type Target = {ctrl_type};
                
                    fn deref(&self) -> &Self::Target {{
                        &self.0
                    }}
                }}
                
                impl DerefMut for {ctrl_name} {{
                    fn deref_mut(&mut self) -> &mut Self::Target {{
                        &mut self.0
                    }}
                }}

                impl TryFrom<ControlValue> for {ctrl_name} {{
                    type Error = ControlValueError;

                    fn try_from(value: ControlValue) -> Result<Self, Self::Error> {{
                        Ok(Self(<{ctrl_type}>::try_from(value)?))
                    }}
                }}

                impl From<{ctrl_name}> for ControlValue {{
                    fn from(val: {ctrl_name}) -> Self {{
                        ControlValue::from(val.0)
                    }}
                }}\n
            "};
        }

        printdoc! {"
            impl ControlEntry for {ctrl_name} {{
                const ID: u32 = {name}::{ctrl_name} as _;
            }}\n
        "};

        match ty {
            ControlsType::Control => println!("impl Control for {ctrl_name} {{}}\n"),
            ControlsType::Property => println!("impl Property for {ctrl_name} {{}}\n"),
        }

        dyn_variants.push_str(&format!(
            "{name}::{ctrl_name} => Ok(Box::new({ctrl_name}::try_from(val)?)),\n"
        ));
    }

    printdoc! {"
        pub fn make_dyn(id: {name}, val: ControlValue) -> Result<Box<dyn DynControlEntry>, ControlValueError> {{
            match id {{
                {dyn_variants}
            }}
        }}
    "};
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        Some("controls") => {
            println!("//! Generated by `cargo run --bin generate_rust controls`\n");
            println!("use std::ops::{{Deref, DerefMut}};");
            println!("use num_enum::{{IntoPrimitive, TryFromPrimitive}};");
            println!("use crate::control::{{Control, ControlEntry, DynControlEntry}};");
            println!("use crate::control_value::{{ControlValue, ControlValueError}};");
            println!("#[allow(unused_imports)]");
            println!("use crate::geometry::{{Rectangle, Size}};\n");
            let controls = control_ids();
            generate_controls(&controls, ControlsType::Control);
        }
        Some("properties") => {
            println!("//! Generated by `cargo run --bin generate_rust properties`\n");
            println!("use std::ops::{{Deref, DerefMut}};");
            println!("use num_enum::{{IntoPrimitive, TryFromPrimitive}};");
            println!("use crate::control::{{Property, ControlEntry, DynControlEntry}};");
            println!("use crate::control_value::{{ControlValue, ControlValueError}};");
            println!("#[allow(unused_imports)]");
            println!("use crate::geometry::{{Rectangle, Size}};\n");
            let properties = property_ids();
            generate_controls(&properties, ControlsType::Property);
        }
        _ => {
            eprintln!("Usage: generate_rust [controls|properties]")
        }
    }
}
