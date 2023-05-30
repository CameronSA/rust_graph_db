use crate::vertex::Vertex;

#[derive(Debug)]
pub enum GraphType {
    InMemory,
    InFile,
}

pub trait Graph {
    fn name(&self) -> &str;

    fn vertices(&self) -> &Vec<Vertex>;

    fn add_vertex(&mut self, vertex: Vertex) -> usize;

    fn list_vertices(&self) -> &Vec<Vertex>;
}

struct InMemoryGraph {
    name: String,
    vertices: Vec<Vertex>,
}

impl Graph for InMemoryGraph {
    fn add_vertex(&mut self, vertex: Vertex) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    fn list_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
}

#[derive(Debug)]
pub enum DataResult {
    CreateGraph(usize),
    ListGraphs(Vec<String>),
}

pub struct GraphFactory {
    pub graphs: Vec<Box<dyn Graph>>,
}

impl GraphFactory {
    pub fn new() -> GraphFactory {
        GraphFactory { graphs: vec![] }
    }

    pub fn create_graph(
        &mut self,
        graph_name: String,
        graph_type: &GraphType,
    ) -> Result<DataResult, String> {
        if graph_name.trim().is_empty() {
            return Err(format!("Must provide a graph name"));
        }

        for graph in &self.graphs {
            if graph.name() == graph_name {
                return Err(format!("Graph with name '{}' already exists", graph.name()));
            }
        }

        match graph_type {
            GraphType::InMemory => {
                let graph = Box::new(InMemoryGraph {
                    name: graph_name,
                    vertices: vec![],
                });
                self.graphs.push(graph);
                Ok(DataResult::CreateGraph(self.graphs.len() - 1))
            }
            _ => {
                let msg = format!("Unexpected graph type: {:?}", graph_type);
                Err(msg)
            }
        }
    }
    
    pub fn list_graphs(&self) -> Result<DataResult, String>{
        let mut graphs = vec![];
        for graph in &self.graphs {
            graphs.push(graph.name().to_string());
        }

        Ok(DataResult::ListGraphs(graphs))
    }

    fn get_graph(&self, graph_name: &str) -> Result<&Box<dyn Graph>, String> 
    {
        let mut graph_ref: Option<&Box<dyn Graph>> = None;
        for graph in &self.graphs {
            if (*graph).name() == graph_name{
                graph_ref = Some(&graph);
                break;
            }
        }

        match graph_ref {
            Some(graph) => Ok(graph),
            None => Err(format!("Unknown graph: {}", graph_name))
        }
    }
}
