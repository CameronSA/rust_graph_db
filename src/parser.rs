mod list_vertices;
mod mutate_vertex;

use crate::executor::{Command, CommandType};
use json::object as JsonObject;

use self::{
    list_vertices::parse_list_vertices_commands, mutate_vertex::parse_vertex_mutation_commmands,
};

const HELP_KEY: &str = "help";
const CREATE_GRAPH_KEY: &str = "createGraph(";
const LIST_GRAPHS_KEY: &str = "listGraph()";
const LIST_VERTICES_KEY: &str = "V()";
const GET_VERTEX_KEY: &str = "V(";
const ADD_VERTEX_KEY: &str = "addV(";
const EDIT_VERTEX_KEY: &str = "editV(";
const EDIT_VERTEX_EMPTY_KEY: &str = "editV()";
const PROPERTY_KEY: &str = "property(";
const REMOVE_PROPERTY_KEY: &str = "removeProperty(";
const HAS_LABEL_KEY: &str = "hasLabel(";
const HAS_PROPERTY_KEY: &str = "hasProperty(";
const HAS_PROPERTY_VALUE_KEY: &str = "hasPropertyValue(";
const HAS_PROPERTY_LIKE_KEY: &str = "hasPropertyLike(";
const VALUES_KEY: &str = "values(";
const END_COMMAND_KEY: &str = ")";

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
    Boolean,
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
            ValidTypes::Boolean => "boolean",
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
    if command.starts_with(CREATE_GRAPH_KEY) && command.ends_with(END_COMMAND_KEY) {
        let graph_name = extract_graph_name(&command_components[0])?;

        command_type = Some(Ok(CommandType::CreateGraph(graph_name)));
    } else if command == LIST_GRAPHS_KEY {
        command_type = Some(Ok(CommandType::ListGraphs));
    } else if command.to_lowercase() == HELP_KEY {
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
        LIST_VERTICES_KEY => Ok(CommandType::ListVertices(Vec::new())),
        _ if command.starts_with(GET_VERTEX_KEY) && command.ends_with(END_COMMAND_KEY) => {
            let vertex_id = extract_number(GET_VERTEX_KEY, command);
            match vertex_id {
                Ok(id) => Ok(CommandType::GetVertex(id)),
                Err(err) => Err(err),
            }
        }

        // Vertex addition
        _ if command.starts_with(ADD_VERTEX_KEY) && command.ends_with(END_COMMAND_KEY) => {
            let vertex_name = extract_string(ADD_VERTEX_KEY, command);
            match vertex_name {
                Ok(name) => Ok(CommandType::AddVertex(name, Vec::new())),
                Err(err) => Err(err),
            }
        }

        // Vertex mutation
        EDIT_VERTEX_EMPTY_KEY => Err(format!("Must provide a vertex id")),
        _ if command.starts_with(EDIT_VERTEX_KEY) && command.ends_with(END_COMMAND_KEY) => {
            let vertex_id = extract_number(EDIT_VERTEX_KEY, command);
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

fn extract_number(key: &str, command: &str) -> Result<usize, String> {
    let stripped_command = extract_string(key, command)?;
    match stripped_command.parse::<usize>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!(
            "Failed to parse value: '{}' as int",
            stripped_command
        )),
    }
}

fn extract_string(key: &str, command: &str) -> Result<String, String> {
    // TODO: guard against commas

    let binding = command.replace(key, "").replace(END_COMMAND_KEY, "");
    let value = binding.trim();
    match value.is_empty() {
        true => Err(format!("Must provide a value for: {key}<value>)")),
        false => Ok(value.to_string()),
    }
}

fn extract_name_value_pair(key: &str, command: &str) -> Result<(String, String), String> {
    let binding = command.replace(key, "").replace(END_COMMAND_KEY, "");
    let values: Vec<&str> = binding.trim().split(",").collect();
    let msg = format!("Invalid format for: {key}<value>). Must provide a name and value pair separated by a comma");
    match values.len() {
        2 => {
            let name = values[0].trim();
            let value = values[1].trim();
            if name.is_empty() || value.is_empty() {
                return Err(msg);
            }

            Ok((name.to_string(), value.to_string()))
        }
        _ => Err(msg),
    }
}

fn extract_graph_name(create_graph_command: &str) -> Result<String, String> {
    let binding = create_graph_command
        .replace(CREATE_GRAPH_KEY, END_COMMAND_KEY)
        .replace(END_COMMAND_KEY, "");
    let name = binding.trim();

    match name.is_empty() {
        true => Err(format!("Must provide a graph name")),
        false => Ok(name.to_string()),
    }
}
