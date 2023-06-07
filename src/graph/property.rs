#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Boolean(bool),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    DateTime(i64), // milliseconds since January 1, 1970
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
    pub flagged_for_removal: bool,
}
