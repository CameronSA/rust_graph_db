use std::str::FromStr;

use crate::graph::DataResult;

#[derive(Debug, Clone, PartialEq)]
pub enum VertexPropertyValue {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    DateTime(i64), // milliseconds since January 1, 1970
}

#[derive(Debug, Clone)]
pub struct VertexProperty {
    pub name: String,
    pub value: VertexPropertyValue,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub label: String,

    // TODO: Enforce that property names are unique
    pub properties: Vec<VertexProperty>,
}

impl Vertex {
    pub fn update(&mut self, properties: Vec<VertexProperty>) -> Result<DataResult, String> {
        if self.properties.len() < 1 {
            self.properties = properties;
            return Ok(DataResult::VertexRef(self));
        }

        // If the property exists, overwrite it. Otherwise, add a new one
        for property in properties {
            let existing_property_index =
                get_first_property_index_by_name(&self.properties, &property.name);

            match existing_property_index {
                Some(index) => self.properties[index] = property,
                None => self.properties.push(property),
            }
        }

        Ok(DataResult::VertexRef(self))
    }

    pub fn has_property(&self, name: &str) -> bool {
        for property in &self.properties {
            if property.name == name {
                return true;
            }
        }

        false
    }

    pub fn has_property_value(&self, name: &str, value: &str) -> bool {
        for property in &self.properties {
            if property.name == name {
                let is_match = match &property.value {
                    VertexPropertyValue::Int32(val) => compare_to_string(*val, value),
                    VertexPropertyValue::Int64(val) => compare_to_string(*val, value),
                    VertexPropertyValue::Float32(val) => compare_to_string(*val, value),
                    VertexPropertyValue::Float64(val) => compare_to_string(*val, value),
                    VertexPropertyValue::String(val) => val == value.trim(),

                    // TODO: This isn't that practical unless searching for a specific epoch
                    VertexPropertyValue::DateTime(val) => compare_to_string(*val, value),
                };

                match is_match {
                    true => return true,
                    false => (),
                }
            }
        }

        false
    }

    /// Only works for string properties
    pub fn has_property_like(&self, name: &str, search_term: &str) -> bool {
        let string_properties = self
            .properties
            .iter()
            .filter(|property| match property.value {
                VertexPropertyValue::String(_) => true,
                _ => false,
            })
            .collect::<Vec<_>>();

        for property in string_properties {
            if property.name == name {
                let is_match = match &property.value {
                    VertexPropertyValue::String(val) => val.contains(search_term.trim()),
                    _ => false,
                };

                match is_match {
                    true => return true,
                    false => (),
                }
            }
        }

        false
    }

    pub fn get_property_value(&self, name: &str) -> Option<&VertexPropertyValue> {
        for property in &self.properties {
            if property.name == name {
                return Some(&property.value);
            }
        }

        None
    }
}

fn compare_to_string<T>(number: T, string: &str) -> bool
where
    T: FromStr + PartialEq,
{
    match string.parse::<T>() {
        Ok(parsed_val) => {
            if parsed_val == number {
                return true;
            }

            false
        }
        Err(_) => false,
    }
}

fn get_first_property_index_by_name<'a>(
    properties: &Vec<VertexProperty>,
    name: &str,
) -> Option<usize> {
    for i in 0..properties.len() {
        if properties[i].name == name {
            return Some(i);
        }
    }

    None
}
