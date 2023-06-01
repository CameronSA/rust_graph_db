#[derive(Debug)]
pub struct VertexProperty {
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct Vertex {
    properties: Vec<VertexProperty>,
}