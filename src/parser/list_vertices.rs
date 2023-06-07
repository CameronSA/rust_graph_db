use crate::{
    executor::VertexFilterCommandType,
    parser::{extract_name_value_pair, extract_value},
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
            _ if command.starts_with("hasLabel(") && command.ends_with(")") => {
                let name = extract_value("hasLabel(", command)?;
                vertex_filter_commands.push(VertexFilterCommandType::HasName(name));
            }

            _ if command.starts_with("hasProperty(") && command.ends_with(")") => {
                let (name, value) = extract_name_value_pair("hasProperty(", command)?;
                vertex_filter_commands.push(VertexFilterCommandType::HasProperty(name, value));
            }

            _ if command.starts_with("hasPropertyLike(") && command.ends_with(")") => {
                let (name, search_term) = extract_name_value_pair("hasPropertyLike(", command)?;
                vertex_filter_commands.push(VertexFilterCommandType::HasPropertyLike(name, search_term));
            }

            _ if command.starts_with("values(") && command.ends_with(")") => {
                let name = extract_value("values(", command)?;
                vertex_filter_commands.push(VertexFilterCommandType::Values(name));
            }

            _ => return Err(format!("Unrecognized vertex filter command: {}", command)),
        }
    }

    Ok(vertex_filter_commands)
}
