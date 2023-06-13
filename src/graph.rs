pub mod edge;
mod entity_map;
pub mod property;
pub mod vertex;

use crate::executor::VertexFilterCommandType;

use self::{edge::Edge, entity_map::EntityMap, property::PropertyValue, vertex::Vertex};

#[derive(Debug)]
pub enum GraphType {
    InMemory,
}

#[derive(Debug)]
pub enum DataResult<'a> {
    UnsignedInt(usize),
    StringVector(Vec<&'a str>),
    VertexIndexVector(Vec<&'a usize>),
    VertexRef(&'a Vertex),
    EdgeRef(&'a Edge),
    MutableVertexRef(&'a mut Vertex),
    VertexValueVector(Vec<Option<&'a PropertyValue>>),
}

pub trait Graph {
    fn name(&self) -> &str;

    fn add_vertex(&mut self, vertex: Vertex) -> Result<DataResult, String>;

    fn add_edge(&mut self, edge: Edge) -> Result<DataResult, String>;

    fn get_vertex(&self, id: &usize) -> Result<DataResult, String>;

    fn remove_vertex(&mut self, id: &usize) -> Result<DataResult, String>;

    fn get_mutable_vertex(&mut self, id: &usize) -> Result<DataResult, String>;

    fn list_vertices(&self, filters: &Vec<VertexFilterCommandType>) -> Result<DataResult, String>;
}

struct InMemoryGraph {
    name: String,
    vertices: EntityMap<Vertex>,
    edges: EntityMap<Edge>,
}

impl Graph for InMemoryGraph {
    fn add_vertex(&mut self, vertex: Vertex) -> Result<DataResult, String> {
        let index = self.vertices.push(vertex);
        Ok(DataResult::UnsignedInt(index))
    }

    fn add_edge(&mut self, edge: Edge) -> Result<DataResult, String> {
        let index = self.edges.push(edge);
        Ok(DataResult::UnsignedInt(index))
    }

    fn list_vertices(&self, filters: &Vec<VertexFilterCommandType>) -> Result<DataResult, String> {
        // TODO: Could have some optimisations here around selecting the order of filter applications
        let mut vertex_indices = self.vertices.get_indices();

        if self.vertices.len() < 1 {
            return Ok(DataResult::VertexIndexVector(vertex_indices));
        }

        let mut vertex_value_vector = Vec::new();

        let mut return_values = false;
        for filter in filters {
            match filter {
                VertexFilterCommandType::HasName(name) => {
                    vertex_indices.retain(|index| match self.vertices.get(index) {
                        Some(val) => &val.label == name,
                        None => false,
                    });
                }
                VertexFilterCommandType::HasProperty(name) => {
                    vertex_indices.retain(|index| match self.vertices.get(index) {
                        Some(val) => val.has_property(name),
                        None => false,
                    });
                }
                VertexFilterCommandType::HasPropertyValue(name, value) => {
                    vertex_indices.retain(|index| match self.vertices.get(index) {
                        Some(val) => val.has_property_value(name, value),
                        None => false,
                    });
                }
                VertexFilterCommandType::HasPropertyLike(name, search_term) => {
                    vertex_indices.retain(|index| match self.vertices.get(index) {
                        Some(val) => val.has_property_like(name, search_term),
                        None => false,
                    });
                }
                VertexFilterCommandType::Values(name) => {
                    for index in &vertex_indices {
                        let value = match self.vertices.get(index) {
                            Some(val) => val.get_property_value(name),
                            None => None,
                        };
                        vertex_value_vector.push(value);
                        return_values = true;
                    }
                }
            }
        }

        match return_values {
            true => Ok(DataResult::VertexValueVector(vertex_value_vector)),
            false => Ok(DataResult::VertexIndexVector(vertex_indices)),
        }
    }

    fn get_vertex(&self, id: &usize) -> Result<DataResult, String> {
        match self.vertices.get(id) {
            Some(val) => Ok(DataResult::VertexRef(val)),
            None => Err(format!("Vertex ID: {} does not exist", id)),
        }
    }

    fn remove_vertex(&mut self, id: &usize) -> Result<DataResult, String> {
        // TODO: Delete any edges attached to the vertex
        match self.vertices.remove(id) {
            Some(_) => Ok(DataResult::UnsignedInt(*id)),
            None => Err(format!("Vertex ID: {} does not exist", id)),
        }
    }

    fn get_mutable_vertex(&mut self, id: &usize) -> Result<DataResult, String> {
        match self.vertices.get_mut(id) {
            Some(val) => Ok(DataResult::MutableVertexRef(val)),
            None => Err(format!("Vertex ID: {} does not exist", id)),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub struct GraphFactory {
    pub graphs: EntityMap<Box<dyn Graph>>,
}

impl GraphFactory {
    pub fn new() -> GraphFactory {
        GraphFactory {
            graphs: EntityMap::new(),
        }
    }

    pub fn create_graph(
        &mut self,
        graph_name: String,
        graph_type: &GraphType,
    ) -> Result<DataResult, String> {
        if graph_name.trim().is_empty() {
            return Err(format!("Must provide a graph name"));
        }

        for (_, graph) in self.graphs.entities() {
            if graph.name() == graph_name {
                return Err(format!("Graph with name '{}' already exists", graph.name()));
            }
        }

        match graph_type {
            GraphType::InMemory => {
                let graph = Box::new(InMemoryGraph {
                    name: graph_name,
                    vertices: EntityMap::new(),
                    edges: EntityMap::new(),
                });
                let index = self.graphs.push(graph);
                Ok(DataResult::UnsignedInt(index))
            }
        }
    }

    pub fn list_graphs(&self) -> Result<DataResult, String> {
        let mut graphs = Vec::new();
        for (_, graph) in self.graphs.entities() {
            graphs.push(graph.name());
        }

        Ok(DataResult::StringVector(graphs))
    }

    pub fn get_graph(&mut self, graph_name: &str) -> Result<&mut Box<dyn Graph>, String> {
        let mut graph_ref: Option<&mut Box<dyn Graph>> = None;
        for (_, graph) in self.graphs.entities_mut() {
            if (*graph).name() == graph_name {
                graph_ref = Some(graph);
                break;
            }
        }

        match graph_ref {
            Some(graph) => Ok(graph),
            None => Err(format!("Unknown graph: {}", graph_name)),
        }
    }
}
