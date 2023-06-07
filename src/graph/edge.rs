use super::{property::Property, vertex::Vertex, DataResult};

#[derive(Debug)]
pub struct Edge {
    pub from_vertex_id: usize,
    pub to_vertex_id: usize,
    pub edge_vertex: Vertex,
}

impl Edge {
    pub fn update(
        &mut self,
        from_vertex_id: usize,
        to_vertex_id: usize,
        properties: Option<Vec<Property>>,
    ) -> Result<DataResult, String> {
        self.from_vertex_id = from_vertex_id;
        self.to_vertex_id = to_vertex_id;

        match properties {
            Some(properties) => match self.edge_vertex.update(properties) {
                Ok(_) => Ok(DataResult::EdgeRef(self)),
                Err(msg) => Err(msg),
            },
            None => Ok(DataResult::EdgeRef(self)),
        }
    }
}
