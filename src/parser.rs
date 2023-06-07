mod list_vertices;
mod mutate_vertex;

use crate::executor::{Command, CommandType};
use json::object as JsonObject;

use self::{mutate_vertex::parse_vertex_mutation_commmands, list_vertices::parse_list_vertices_commands};

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

pub fn parse(command: String) -> Result<Command, String> {
    let command_components: Vec<&str> = command.split(".").collect();

    // Graph commands
    let mut command_type = None;
    if command.starts_with("createGraph(") && command.ends_with(")") {
        let graph_name = extract_graph_name(&command_components[0])?;

        command_type = Some(Ok(CommandType::CreateGraph(graph_name)));
    } else if command == "listGraphs()" {
        command_type = Some(Ok(CommandType::ListGraphs));
    } else if command.to_lowercase() == "help" {
        command_type = Some(Ok(CommandType::Help));
    }

    // Function determination
    let command_type = match command_type {
        Some(val) => val,
        None => get_command_type(&command_components),
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

            CommandType::ListVertices(filter_command) => Ok(Command {
                command_type: CommandType::ListVertices(filter_command),
                command_json: Some(JsonObject! {
                    graph_name: identify_graph(&command_components)
                }),
            }),

            CommandType::GetVertex(id) => Ok(Command {
                command_type: CommandType::GetVertex(id),
                command_json: Some(JsonObject! {
                    graph_name: identify_graph(&command_components)
                }),
            }),

            CommandType::AddVertex(label, mutation_command) => Ok(Command {
                command_type: CommandType::AddVertex(label, mutation_command),
                command_json: Some(JsonObject! {
                    graph_name: identify_graph(&command_components)
                }),
            }),

            CommandType::EditVertex(id, mutation_command) => Ok(Command {
                command_type: CommandType::EditVertex(id, mutation_command),
                command_json: Some(JsonObject! {
                    graph_name: identify_graph(&command_components)
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
        "V()" => Ok(CommandType::ListVertices(Vec::new())),
        _ if command.starts_with("V(") && command.ends_with(")") => {
            let vertex_id = extract_vertex_id("V(", command);
            match vertex_id {
                Ok(id) => Ok(CommandType::GetVertex(id)),
                Err(err) => Err(err),
            }
        }

        // Vertex addition
        _ if command.starts_with("addV(") && command.ends_with(")") => {
            let vertex_name = extract_value("addV(", command);
            match vertex_name {
                Ok(name) => Ok(CommandType::AddVertex(name, Vec::new())),
                Err(err) => Err(err),
            }
        }

        // Vertex mutation
        "editV()" => Err(format!("Must provide a vertex id")),
        _ if command.starts_with("editV(") && command.ends_with(")") => {
            let vertex_id = extract_vertex_id("editV(", command);
            match vertex_id {
                Ok(id) => Ok(CommandType::EditVertex(id, Vec::new())),
                Err(err) => Err(err),
            }
        }

        // Catch all
        _ => Err(format!("Unrecognized command: {}", command)),
    }?;

    // Add follow up commands
    match command_type {
        CommandType::ListVertices(_) => {
            let filter_commands = parse_list_vertices_commands(command_components)?;
            command_type = CommandType::ListVertices(filter_commands);
        }
        CommandType::AddVertex(label, _) => {
            let mutation_commands = parse_vertex_mutation_commmands(command_components)?;
            command_type = CommandType::AddVertex(label, mutation_commands);
        }
        CommandType::EditVertex(id, _) => {
            let mutation_commands = parse_vertex_mutation_commmands(command_components)?;
            command_type = CommandType::EditVertex(id, mutation_commands);
        }

        _ => (),
    };

    Ok(command_type)
}

fn extract_vertex_id(key: &str, command: &str) -> Result<usize, String> {
    let stripped_command = extract_value(key, command)?;
    match stripped_command.parse::<usize>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!(
            "Failed to parse value: '{}' as int",
            stripped_command
        )),
    }
}

fn extract_value(key: &str, command: &str) -> Result<String, String> {
    let binding = command.replace(key, "").replace(")", "");
    let value = binding.trim();
    match value.is_empty() {
        true => Err(format!("Must provide a value for: {key}<value>)")),
        false => Ok(value.to_string()),
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
