use crate::{
    executor::{Command, CommandType, VertexMutationCommandType},
    vertex::{VertexProperty, VertexPropertyValue},
};
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

pub enum ValidTypes {
    Int32,
    Int64,
    Float32,
    Float64,
    String,
    DateTime,
}

impl ValidTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValidTypes::Int32 => "int32",
            ValidTypes::Int64 => "int64",
            ValidTypes::Float32 => "float32",
            ValidTypes::Float64 => "float64",
            ValidTypes::String => "string",
            ValidTypes::DateTime => "datetime",
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
        let mut command_type = match command {
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
        }?;

        // Add follow up commands
        match command_type {
            CommandType::ListVertices => todo!(),
            CommandType::AddVertex(_) => {
                let mutation_commands = Self::parse_vertex_mutation_commmands(command_components)?;
                command_type = CommandType::AddVertex(mutation_commands);
            }
            _ => (),
        };

        Ok(command_type)
    }

    fn parse_vertex_mutation_commmands(
        commands: &Vec<&str>,
    ) -> Result<Vec<VertexMutationCommandType>, String> {
        // First command is graph, second is vertex. So, vertex mutations occur from the third command
        if commands.len() < 3 {
            return Ok(Vec::new());
        }

        let mut vertex_mutation_commands = Vec::new();
        for command in commands.iter().skip(2) {
            match command {
                _ if command.starts_with("property(") && command.ends_with(")") => {
                    let property = Self::parse_add_vertex_property_command(command)?;
                    let vertex_mutation_command = VertexMutationCommandType::Property(property);
                    vertex_mutation_commands.push(vertex_mutation_command);
                }

                _ => return Err(format!("Unrecognized vertex mutation command: {}", command)),
            }
        }

        Ok(vertex_mutation_commands)
    }

    fn parse_add_vertex_property_command(command: &str) -> Result<VertexProperty, String> {
        let stripped_command = command.replace("property(", "").replace(")", "");
        let stripped_command_components: Vec<&str> = stripped_command.split(",").collect();

        // Property name, value and type
        if stripped_command_components.len() != 3 {
            return Err(format!("Parsed vertex command components as '{:?}'. This is invalid, must have a property name, value, and value type", stripped_command_components));
        }

        let property_name = stripped_command_components[0].trim();
        let property_value_str = stripped_command_components[1].trim();
        let property_type_str = stripped_command_components[2].trim();

        // Validate value and type
        let property_value = match property_type_str {
            _ if property_type_str == ValidTypes::Int32.as_str() => {
                match property_value_str.parse::<i32>() {
                    Ok(value) => VertexPropertyValue::Int32(value),
                    Err(_) => {
                        return Err(format!(
                            "Failed to parse value: {} as {}",
                            property_value_str,
                            ValidTypes::Int32.as_str()
                        ))
                    }
                }
            }
            _ if property_type_str == ValidTypes::Int64.as_str() => {
                match property_value_str.parse::<i64>() {
                    Ok(value) => VertexPropertyValue::Int64(value),
                    Err(_) => {
                        return Err(format!(
                            "Failed to parse value: {} as {}",
                            property_value_str,
                            ValidTypes::Int64.as_str()
                        ))
                    }
                }
            }
            _ if property_type_str == ValidTypes::Float32.as_str() => {
                match property_value_str.parse::<f32>() {
                    Ok(value) => VertexPropertyValue::Float32(value),
                    Err(_) => {
                        return Err(format!(
                            "Failed to parse value: {} as {}",
                            property_value_str,
                            ValidTypes::Float32.as_str()
                        ))
                    }
                }
            }
            _ if property_type_str == ValidTypes::Float64.as_str() => {
                match property_value_str.parse::<f64>() {
                    Ok(value) => VertexPropertyValue::Float64(value),
                    Err(_) => {
                        return Err(format!(
                            "Failed to parse value: {} as {}",
                            property_value_str,
                            ValidTypes::Float64.as_str()
                        ))
                    }
                }
            }
            _ if property_type_str == ValidTypes::String.as_str() => {
                VertexPropertyValue::String(property_value_str.to_string())
            }
            _ if property_type_str == ValidTypes::DateTime.as_str() => {
                match property_value_str.parse::<i64>() {
                    Ok(value) => VertexPropertyValue::DateTime(value),
                    Err(_) => {
                        return Err(format!(
                            "Failed to parse value: {} as {}",
                            property_value_str,
                            ValidTypes::DateTime.as_str()
                        ))
                    }
                }
            }
            _ => return Err(format!("Unrecognized type: {}", property_type_str)),
        };

        Ok(VertexProperty {
            name: property_name.to_string(),
            value: property_value,
        })
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
