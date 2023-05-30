use json::JsonValue as Json;

use crate::graph::{GraphFactory, GraphType};

#[derive(Debug)]
pub enum CommandType {
    CreateGraph(String),
    ListGraphs(), 
    ListVertices,
    GetVertex(usize),
    AddVertex,
}

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub command_json: Option<Json>,
}

pub struct Executor {
    graph_factory: GraphFactory,
    graph_type: GraphType
}

impl Executor {
    pub fn new(graph_factory: GraphFactory, graph_type: GraphType) -> Self {
        Executor {
            graph_factory,
            graph_type,
        }
    }

    pub fn execute(&mut self, command: Command) {
        println!("Executing: {:?}", command);

        let result = match command.command_type {
            CommandType::CreateGraph(graph_name) => self.graph_factory.create_graph(graph_name, &self.graph_type),

            CommandType::ListGraphs() => self.graph_factory.list_graphs(),
            _ => todo!(),
        };
        
        match result {
            Ok(result) => println!("{:?}", result),
            Err(err) => println!("{}", err)
        }
    }
}
