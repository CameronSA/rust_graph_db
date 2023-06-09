use std::str::FromStr;

use crate::graph::DataResult;

use super::property::{Property, PropertyValue};

#[derive(Debug, Clone)]
pub struct Vertex {
    pub label: String,
    pub properties: Vec<Property>,
    pub out_edge_ids: Vec<usize>,
    pub in_edge_ids: Vec<usize>,
}

impl Vertex {
    pub fn new(label: String, properties: Vec<Property>) -> Self {
        Vertex {
            label,
            properties,
            in_edge_ids: Vec::new(),
            out_edge_ids: Vec::new(),
        }
    }

    pub fn update(&mut self, properties: Vec<Property>) -> Result<DataResult, String> {
        if self.properties.len() < 1 {
            self.properties = properties;
            return Ok(DataResult::VertexRef(self));
        }

        let mut property_indices_to_remove = Vec::new();

        for property in properties {
            let existing_property_index =
                get_first_property_index_by_name(&self.properties, &property.name);

            match existing_property_index {
                Some(index) => match property.flagged_for_removal {
                    true => {
                        if !property_indices_to_remove.contains(&index) {
                            property_indices_to_remove.push(index)
                        }
                    }
                    false => self.properties[index] = property,
                },
                None => self.properties.push(property),
            }
        }

        // sort by descending
        property_indices_to_remove.sort_by(|a, b| b.cmp(a));

        for index in property_indices_to_remove {
            self.properties.remove(index);
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
                    PropertyValue::Boolean(val) => compare_to_string(*val, value),
                    PropertyValue::Int32(val) => compare_to_string(*val, value),
                    PropertyValue::Int64(val) => compare_to_string(*val, value),
                    PropertyValue::Float32(val) => compare_to_string(*val, value),
                    PropertyValue::Float64(val) => compare_to_string(*val, value),
                    PropertyValue::String(val) => val == value.trim(),

                    // TODO: This isn't that practical unless searching for a specific epoch
                    PropertyValue::DateTime(val) => compare_to_string(*val, value),
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
                PropertyValue::String(_) => true,
                _ => false,
            })
            .collect::<Vec<_>>();

        for property in string_properties {
            if property.name == name {
                let is_match = match &property.value {
                    PropertyValue::String(val) => val.contains(search_term.trim()),
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

    pub fn get_property_value(&self, name: &str) -> Option<&PropertyValue> {
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

fn get_first_property_index_by_name<'a>(properties: &Vec<Property>, name: &str) -> Option<usize> {
    for i in 0..properties.len() {
        if properties[i].name == name {
            return Some(i);
        }
    }

    None
}
