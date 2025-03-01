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

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "bool" => Ok(Self::Bool),
            "byte" | "uint8" | "uint8_t" | "u8" => Ok(Self::Byte),
            "int32" | "int32_t" => Ok(Self::Int32),
            "int64" | "int64_t" => Ok(Self::Int64),
            "float" => Ok(Self::Float),
            "string" => Ok(Self::String),
            "rectangle" => Ok(Self::Rectangle),
            "size" => Ok(Self::Size),
            "point" => Ok(Self::Point),
            _ => Err(format!("Unknown control type {s}")),
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
