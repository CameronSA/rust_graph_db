use super::{
    property::{Property, PropertyValue},
    vertex::Vertex,
    DataResult,
};

#[derive(Debug, Clone)]
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

    pub fn has_property(&self, name: &str) -> bool {
        self.edge_vertex.has_property(name)
    }

    pub fn has_property_value(&self, name: &str, value: &str) -> bool {
        self.edge_vertex.has_property_value(name, value)
    }

    pub fn has_property_like(&self, name: &str, search_term: &str) -> bool {
        self.edge_vertex.has_property_like(name, search_term)
    }

    pub fn get_property_value(&self, name: &str) -> Option<&PropertyValue> {
        self.edge_vertex.get_property_value(name)
    }
}
