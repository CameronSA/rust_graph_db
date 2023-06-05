use crate::graph::DataResult;

#[derive(Debug, Clone)]
pub enum VertexPropertyValue {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    DateTime(i64), // milliseconds since January 1, 1970
}

#[derive(Debug)]
pub struct VertexProperty {
    pub name: String,
    pub value: VertexPropertyValue,
}

#[derive(Debug)]
pub struct Vertex {
    pub label: String,
    pub properties: Vec<VertexProperty>,
}

impl Vertex {
    pub fn update(&mut self, properties: Vec<VertexProperty>) -> Result<DataResult, String> {
        self.properties = properties;
        Ok(DataResult::VertexRef(self))
    }
}
