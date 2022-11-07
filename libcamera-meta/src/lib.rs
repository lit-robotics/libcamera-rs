use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub const CONTROL_IDS_YAML: &'static str = include_str!("./control_ids.yaml");
pub const PROPERTY_IDS_YAML: &'static str = include_str!("./property_ids.yaml");
pub const FORMATS_YAML: &'static str = include_str!("./formats.yaml");

#[derive(Serialize, Deserialize, Debug)]
pub struct Control {
    #[serde(rename = "type")]
    typ: String,
    description: String,
    #[serde(rename = "enum")]
    enumeration: Option<Vec<ControlEnumValue>>,
    size: Option<Vec<usize>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlEnumValue {
    name: String,
    value: i32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlEnumValue {
    name: String,
    value: i32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Controls {
    controls: Vec<BTreeMap<String, Control>>,
}

pub fn control_ids() -> Controls {
    serde_yaml::from_str(CONTROL_IDS_YAML).unwrap()
}
