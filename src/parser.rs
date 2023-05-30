use crate::executor::{Command, CommandType};
use json::object as JsonObject;

pub struct Parser;
impl Parser {
    pub fn parse(command: String) -> Result<Command, String> {
        // let obj = JsonObject!{
        // test:true
        // };

        let command_components: Vec<&str> = command.split(".").collect();

        // Graph creation
        let mut command_type = None;
        if command.starts_with("createGraph(") && command.ends_with(")") {
            let graph_name = match Self::extract_graph_name(&command_components[0]) {
                Ok(name) => name,
                Err(err) => return Err(err),
            };

            command_type = Some(Ok(CommandType::CreateGraph(graph_name)));
        }

        // Function determination
        let command_type = match command_type {
            Some(val) => val,
            None => Self::get_command_type(&command_components),
        };

        match command_type {
            Ok(command_type) => match command_type {
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

                CommandType::AddVertex => Ok(Command {
                    command_type: CommandType::AddVertex,
                    command_json: Some(JsonObject! {
                        graph_name: Self::identify_graph(&command_components)
                    }),
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
            // Graph creation
            _ if command.starts_with("createGraph(") && command.ends_with(")") => {
                let graph_name = match Self::extract_graph_name(command) {
                    Ok(name) => name,
                    Err(err) => return Err(err),
                };

                Ok(CommandType::CreateGraph(graph_name))
            }

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
            "addV()" => Ok(CommandType::AddVertex),

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

        if name.is_empty() {
            return Err(format!("Must provide a graph name"));
        } else {
            return Ok(name.to_string());
        }
    }
}
