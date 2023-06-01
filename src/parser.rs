use crate::executor::{Command, CommandType};
use json::object as JsonObject;

pub enum JsonProperty {
    GraphName,
}

impl JsonProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            JsonProperty::GraphName => "graph_name",
        }
    }
}

pub struct Parser;
impl Parser {
    pub fn parse(command: String) -> Result<Command, String> {
        let command_components: Vec<&str> = command.split(".").collect();

        // Graph commands
        let mut command_type = None;
        if command.starts_with("createGraph(") && command.ends_with(")") {
            let graph_name = Self::extract_graph_name(&command_components[0])?;

            command_type = Some(Ok(CommandType::CreateGraph(graph_name)));
        } else if command == "listGraphs()" {
            command_type = Some(Ok(CommandType::ListGraphs));
        } else if command.to_lowercase() == "help" {
            command_type = Some(Ok(CommandType::Help));
        }

        // Function determination
        let command_type = match command_type {
            Some(val) => val,
            None => Self::get_command_type(&command_components),
        };

        match command_type {
            Ok(command_type) => match command_type {
                CommandType::ListGraphs => Ok(Command {
                    command_type: CommandType::ListGraphs,
                    command_json: None,
                }),

                CommandType::CreateGraph(name) => Ok(Command {
                    command_type: CommandType::CreateGraph(name),
                    command_json: None,
                }),

                CommandType::ListVertices => Ok(Command {
                    command_type: CommandType::ListVertices,
                    command_json: Some(JsonObject! {
                        graph_name: Self::identify_graph(&command_components)
                    }),
                }),

                CommandType::GetVertex(id) => Ok(Command {
                    command_type: CommandType::GetVertex(id),
                    command_json: Some(JsonObject! {
                        graph_name: Self::identify_graph(&command_components)
                    }),
                }),

                CommandType::AddVertex(mutation_command) => Ok(Command {
                    command_type: CommandType::AddVertex(mutation_command),
                    command_json: Some(JsonObject! {
                        graph_name: Self::identify_graph(&command_components)
                    }),
                }),

                CommandType::Help => Ok(Command {
                    command_type: CommandType::Help,
                    command_json: None,
                }),
            },
            Err(err) => Err(err),
        }
    }

    fn identify_graph(command_components: &Vec<&str>) -> String {
        command_components[0].trim().to_string()
    }

    fn get_command_type(command_components: &Vec<&str>) -> Result<CommandType, String> {
        if command_components.len() < 2 {
            return Err(format!("Not enough command components"));
        }

        let command = command_components[1].trim();
        match command {
            // Vertex selection
            "V()" => Ok(CommandType::ListVertices),
            _ if command.starts_with("V(") && command.ends_with(")") => {
                let vertex_id = Self::extract_vertex_id(command);
                match vertex_id {
                    Ok(id) => Ok(CommandType::GetVertex(id)),
                    Err(err) => Err(err),
                }
            }

            // Vertex addition
            "addV()" => Ok(CommandType::AddVertex(Vec::new())),

            // Catch all
            _ => Err(format!("Unrecognized command: {}", command)),
        }
    }

    fn extract_vertex_id(get_vertex_command: &str) -> Result<usize, String> {
        let stripped_command = get_vertex_command.replace("V(", "").replace(")", "");
        match stripped_command.parse::<usize>() {
            Ok(num) => Ok(num),
            Err(_) => Err(format!(
                "Failed to parse value: '{}' as int",
                stripped_command
            )),
        }
    }

    fn extract_graph_name(create_graph_command: &str) -> Result<String, String> {
        let binding = create_graph_command
            .replace("createGraph(", ")")
            .replace(")", "");
        let name = binding.trim();

        match name.is_empty() {
            true => Err(format!("Must provide a graph name")),
            false => Ok(name.to_string()),
        }
    }
}
