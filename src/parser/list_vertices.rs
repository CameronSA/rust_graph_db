use crate::{
    executor::VertexFilterCommandType,
    parser::{extract_name_value_pair, extract_string},
};

use super::{
    END_COMMAND_KEY, HAS_LABEL_KEY, HAS_PROPERTY_KEY, HAS_PROPERTY_LIKE_KEY,
    HAS_PROPERTY_VALUE_KEY, VALUES_KEY,
};

pub fn parse_list_vertices_commands(
    commands: &Vec<&str>,
) -> Result<Vec<VertexFilterCommandType>, String> {
    // First command is graph, second is vertex. So, vertex filters occur from the third command
    if commands.len() < 3 {
        return Ok(Vec::new());
    }

    let mut vertex_filter_commands = Vec::new();
    for command in commands.iter().skip(2) {
        match command {
            _ if command.starts_with(HAS_LABEL_KEY) && command.ends_with(END_COMMAND_KEY) => {
                let name = extract_string(HAS_LABEL_KEY, command)?;
                vertex_filter_commands.push(VertexFilterCommandType::HasName(name));
            }

            _ if command.starts_with(HAS_PROPERTY_KEY) && command.ends_with(END_COMMAND_KEY) => {
                let name = extract_string(HAS_PROPERTY_KEY, command)?;
                vertex_filter_commands.push(VertexFilterCommandType::HasProperty(name));
            }

            _ if command.starts_with(HAS_PROPERTY_VALUE_KEY)
                && command.ends_with(END_COMMAND_KEY) =>
            {
                let (name, value) = extract_name_value_pair(HAS_PROPERTY_VALUE_KEY, command)?;
                vertex_filter_commands.push(VertexFilterCommandType::HasPropertyValue(name, value));
            }

            _ if command.starts_with(HAS_PROPERTY_LIKE_KEY)
                && command.ends_with(END_COMMAND_KEY) =>
            {
                let (name, search_term) = extract_name_value_pair(HAS_PROPERTY_LIKE_KEY, command)?;
                vertex_filter_commands
                    .push(VertexFilterCommandType::HasPropertyLike(name, search_term));
            }

            _ if command.starts_with(VALUES_KEY) && command.ends_with(END_COMMAND_KEY) => {
                // TODO: ability to select multiple properties
                let name = extract_string(VALUES_KEY, command)?;
                vertex_filter_commands.push(VertexFilterCommandType::Values(name));
            }

            _ => return Err(format!("Unrecognized vertex filter command: {}", command)),
        }
    }

    Ok(vertex_filter_commands)
}
