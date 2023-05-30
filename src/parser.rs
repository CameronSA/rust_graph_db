use std::{num::ParseIntError, fmt::format};
use json::{object as JsonObject, JsonValue as Json};
use crate::graph::{Graph, GraphFactory};

#[derive(Debug)]
enum CommandType {
    CreateGraph(String),
    ListVertices,
    GetVertex(usize),
    AddVertex,
}

pub struct Command {
    command_type: CommandType,
    command_json: Option<Json>,
}

pub struct Parser;
impl Parser {
    pub fn parse(command: String) -> Result<Command, String> {
        // let obj = JsonObject!{
        // test:true
        // };

        let command_components: Vec<&str> = command.split(".").collect();
        if command_components.len() < 1 {
            return Err(format!("Please provide a command"));
        }

        let command_type = Self::get_command_type(command_components[0]);

        match command_type {
            Ok(command_type) => { 
                match command_type {
                    CommandType::CreateGraph(name) => {
                        Ok(Command { command_type: CommandType::CreateGraph(name), command_json: None })
                    },

                    _ => Err(format!("Command: {:?} not implemented", command_type))
                }
            },
            Err(err) => Err(err)
        }
    }   

    fn get_command_type(command: &str) -> Result<CommandType, String> {
        match command {
            // Graph creation
            _ if command.starts_with("createGraph(") && command.ends_with(")") => {
                let graph_name = Self::extract_graph_name(command);
                Ok(CommandType::CreateGraph(graph_name))
            }

            // Vertex selection
            "V()" => Ok(CommandType::ListVertices),
            _ if command.starts_with("V(") && command.ends_with(")") => {
                let vertex_id = Self::extract_vertex_id(command);
                match vertex_id {
                    Ok(id) => Ok(CommandType::GetVertex(id)),
                    Err(err) => Err(err.to_string()),
                }
            }

            // Catch all
            _ => Err(format!("Unrecognized command: {}", command))
        }
    }

    fn extract_vertex_id(get_vertex_command: &str) -> Result<usize, ParseIntError> {
        let stripped_command = get_vertex_command.replace("V(", "").replace(")", "");
        stripped_command.parse::<usize>()
    }

    fn extract_graph_name(create_graph_command: &str) -> String {
        create_graph_command.replace("createGraph(", ")").replace(")", "")
    }
}
