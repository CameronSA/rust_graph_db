use super::{extract_string, ValidTypes, END_COMMAND_KEY, PROPERTY_KEY, REMOVE_PROPERTY_KEY};
use crate::{
    executor::VertexMutationCommandType,
    graph::property::{Property, PropertyValue},
};

pub fn parse_entity_mutation_commmands(
    commands: &Vec<&str>,
) -> Result<Vec<VertexMutationCommandType>, String> {
    // First command is graph, second is entity. So, entity mutations occur from the third command
    if commands.len() < 3 {
        return Ok(Vec::new());
    }

    let mut vertex_mutation_commands = Vec::new();
    for command in commands.iter().skip(2) {
        match command {
            _ if command.starts_with(PROPERTY_KEY) && command.ends_with(END_COMMAND_KEY) => {
                let property = parse_add_vertex_property_command(command)?;
                let vertex_mutation_command = VertexMutationCommandType::Property(property);
                vertex_mutation_commands.push(vertex_mutation_command);
            }

            _ if command.starts_with(REMOVE_PROPERTY_KEY) && command.ends_with(END_COMMAND_KEY) => {
                let property_name = extract_string(REMOVE_PROPERTY_KEY, command)?;
                let vertex_mutation_command =
                    VertexMutationCommandType::RemoveProperty(property_name);
                vertex_mutation_commands.push(vertex_mutation_command);
            }

            _ => return Err(format!("Unrecognized vertex mutation command: {}", command)),
        }
    }

    Ok(vertex_mutation_commands)
}

fn parse_add_vertex_property_command(command: &str) -> Result<Property, String> {
    let stripped_command = command
        .replace(PROPERTY_KEY, "")
        .replace(END_COMMAND_KEY, "");
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
        _ if property_type_str == ValidTypes::Boolean.as_str() => {
            match property_value_str.parse::<bool>() {
                Ok(value) => PropertyValue::Boolean(value),
                Err(_) => {
                    return Err(format!(
                        "Failed to parse value: {} as {}",
                        property_value_str,
                        ValidTypes::Boolean.as_str()
                    ))
                }
            }
        }

        _ if property_type_str == ValidTypes::Int32.as_str() => {
            match property_value_str.parse::<i32>() {
                Ok(value) => PropertyValue::Int32(value),
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
                Ok(value) => PropertyValue::Int64(value),
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
                Ok(value) => PropertyValue::Float32(value),
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
                Ok(value) => PropertyValue::Float64(value),
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
            PropertyValue::String(property_value_str.to_string())
        }
        _ if property_type_str == ValidTypes::DateTime.as_str() => {
            // TODO: Not user friendly to input ms since epoch
            match property_value_str.parse::<i64>() {
                Ok(value) => PropertyValue::DateTime(value),
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

    Ok(Property {
        name: property_name.to_string(),
        value: property_value,
        flagged_for_removal: false,
    })
}
