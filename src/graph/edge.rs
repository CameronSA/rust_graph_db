use super::{vertex::Vertex, property::Property};

#[derive(Debug)]
pub struct Edge<'a> {
    pub from_vertex: &'a Vertex,
    pub to_vertex: &'a Vertex,
    pub label: String,
    pub properties: Vec<Property>,
}
