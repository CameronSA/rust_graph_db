mod vertex;
use vertex::Vertex;

#[derive(Debug)]
enum GraphType {
    InMemory,
    InFile,
}

trait Graph {
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
}

struct GraphFactory {
    graphs: Vec<Box<dyn Graph>>,
}

impl GraphFactory {
    fn new() -> GraphFactory {
        GraphFactory { graphs: vec![] }
    }

    fn create_graph(
        &mut self,
        graph_name: String,
        graph_type: &GraphType,
    ) -> Result<usize, String> {
        if graph_name.trim().is_empty() {
            return Err(format!("Must provide a graph name"));
        }

        match graph_type {
            GraphType::InMemory => {
                let graph = Box::new(InMemoryGraph {
                    name: graph_name,
                    vertices: vec![],
                });
                self.graphs.push(graph);
                Ok(self.graphs.len() - 1)
            }
            _ => {
                let msg = format!("Unexpected graph type: {:?}", graph_type);
                Err(msg)
            }
        }
    }
}
