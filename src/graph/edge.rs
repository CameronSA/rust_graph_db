use super::vertex::Vertex;

#[derive(Debug)]
pub struct Edge<'a> {
    pub from_vertex: &'a Vertex,
    pub to_vertex: &'a Vertex,
    pub edge_vertex: Vertex,
}
