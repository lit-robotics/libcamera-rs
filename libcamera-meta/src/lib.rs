use yaml_rust::Yaml;

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
    Point,
}

impl TryFrom<&str> for ControlType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bool" => Ok(ControlType::Bool),
            "uint8_t" => Ok(ControlType::Byte),
            "int32_t" => Ok(ControlType::Int32),
            "int64_t" => Ok(ControlType::Int64),
            "float" => Ok(ControlType::Float),
            "string" => Ok(ControlType::String),
            "Rectangle" => Ok(ControlType::Rectangle),
            "Size" => Ok(ControlType::Size),
            "Point" => Ok(ControlType::Point),
            _ => Err(format!("Unknown control type {}", value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ControlSize {
    Dynamic,
    Fixed(usize),
}

impl TryFrom<&Yaml> for ControlSize {
    type Error = String;

    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        match value {
            Yaml::Integer(size) => match (*size).try_into() {
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
pub struct ControlEnumValue {
    pub name: String,
    pub value: i32,
    pub description: String,
}
