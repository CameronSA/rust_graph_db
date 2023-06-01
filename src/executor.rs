use json::JsonValue as Json;

use crate::{
    graph::{DataResult, Graph, GraphFactory, GraphType},
    parser::JsonProperty,
    vertex::Vertex,
};

#[derive(Debug)]
pub enum CommandType {
    CreateGraph(String),
    ListGraphs,
    ListVertices,
    GetVertex(usize),
    AddVertex,
    Help,
}

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub command_json: Option<Json>,
}

pub struct Executor {
    graph_factory: GraphFactory,
    graph_type: GraphType,
}

impl Executor {
    pub fn new(graph_factory: GraphFactory, graph_type: GraphType) -> Self {
        Executor {
            graph_factory,
            graph_type,
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<DataResult, String> {
        println!("Executing: {:?}", command);

        match command.command_type {
            CommandType::CreateGraph(graph_name) => self
                .graph_factory
                .create_graph(graph_name, &self.graph_type),

            CommandType::ListGraphs => self.graph_factory.list_graphs(),

            CommandType::ListVertices => {
                let graph = self.get_graph(command)?;
                graph.list_vertices()
            }

            CommandType::GetVertex(id) => {
                let graph = self.get_graph(command)?;
                graph.get_vertex(id)
            }

            CommandType::AddVertex => {
                let graph = self.get_mut_graph(command)?;
                let vertex = Vertex {};
                graph.add_vertex(vertex)
            }

            _ => todo!(),
        }
    }

    fn get_mut_graph(&mut self, command: Command) -> Result<&mut Box<dyn Graph>, String> {
        let graph_name = self.get_graph_name(command)?;

        self.graph_factory.get_graph(&graph_name)
    }

    fn get_graph(&mut self, command: Command) -> Result<&Box<dyn Graph>, String> {
        let graph = self.get_mut_graph(command)?;
        Ok(&*graph)
    }

    fn get_graph_name(&self, command: Command) -> Result<String, String> {
        let msg = "Graph not specified".to_string();

        match command.command_json {
            Some(json) => {
                let name = json[JsonProperty::GraphName.as_str()].clone();
                match name.is_null() {
                    true => Err(msg),
                    false => Ok(name.to_string()),
                }
            }
            None => return Err(msg),
        }
    }
}
