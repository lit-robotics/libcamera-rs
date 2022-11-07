use yaml_rust::{Yaml, YamlLoader};

pub const CONTROL_IDS_YAML: &'static str = include_str!("./control_ids.yaml");
pub const PROPERTY_IDS_YAML: &'static str = include_str!("./property_ids.yaml");
pub const FORMATS_YAML: &'static str = include_str!("./formats.yaml");

#[derive(Debug, Clone, Copy)]
pub enum ControlType {
    Bool,
    Byte,
    Int32,
    Int64,
    Float,
    String,
    Rectangle,
    Size,
}

impl TryFrom<&str> for ControlType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bool" => Ok(ControlType::Bool),
            "int32_t" => Ok(ControlType::Int32),
            "int64_t" => Ok(ControlType::Int64),
            "float" => Ok(ControlType::Float),
            "string" => Ok(ControlType::String),
            "Rectangle" => Ok(ControlType::Rectangle),
            "Size" => Ok(ControlType::Size),
            _ => Err(format!("Unknown control type {}", value)),
        }
    }
}

#[derive(Debug)]
pub enum ControlSize {
    Dynamic,
    Fixed(usize),
}

impl TryFrom<&Yaml> for ControlSize {
    type Error = String;

    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        match value {
            Yaml::Integer(size) => match size.clone().try_into() {
                Ok(size) => Ok(ControlSize::Fixed(size)),
                _ => Err(format!("Invalid ControlSize integer {}", size)),
            },
            Yaml::String(s) => match s.as_str() {
                "n" => Ok(ControlSize::Dynamic),
                _ => Err(format!("Unknown ControlSize string {}", s)),
            },
            _ => Err(format!("Unknown ControlSize type {:?}", value)),
        }
    }
}

#[derive(Debug)]
pub struct Control {
    pub name: String,
    pub typ: ControlType,
    pub description: String,
    pub size: Option<Vec<ControlSize>>,
    pub enumeration: Option<Vec<ControlEnumValue>>,
}

#[derive(Debug)]
pub struct ControlEnumValue {
    pub name: String,
    pub value: i32,
    pub description: String,
}

fn parse_controls(yaml: &str) -> Vec<Control> {
    let doc = YamlLoader::load_from_str(yaml).unwrap();
    let base = &doc[0];
    let controls = &base["controls"];

    controls
        .as_vec()
        .unwrap()
        .into_iter()
        .map(|control| {
            let (name, val) = control.as_hash().unwrap().front().unwrap();

            let name = name.as_str().unwrap().to_string();
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

            Control {
                name,
                typ,
                description,
                size,
                enumeration,
            }
        })
        .collect()
}

pub fn control_ids() -> Vec<Control> {
    parse_controls(CONTROL_IDS_YAML)
}

pub fn property_ids() -> Vec<Control> {
    parse_controls(PROPERTY_IDS_YAML)
}
