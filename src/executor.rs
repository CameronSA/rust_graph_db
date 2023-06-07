use json::JsonValue as Json;

use crate::{
    graph::{
        property::{Property, PropertyValue},
        vertex::Vertex,
        DataResult, Graph, GraphFactory, GraphType,
    },
    parser::JsonProperty,
};

#[derive(Debug)]
pub enum VertexMutationCommandType {
    Property(Property),
    RemoveProperty(String),
}

#[derive(Debug)]
pub enum VertexFilterCommandType {
    HasName(String),
    HasProperty(String),              // name
    HasPropertyValue(String, String), // name, value pair
    HasPropertyLike(String, String),  // name, search term pair
    Values(String),                   // property name
}

#[derive(Debug)]
pub enum CommandType {
    CreateGraph(String),
    ListGraphs,
    ListVertices(Vec<VertexFilterCommandType>),
    GetVertex(usize),
    AddVertex(String, Vec<VertexMutationCommandType>),
    EditVertex(usize, Vec<VertexMutationCommandType>),
    Help,
}

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub command_json: Option<Json>,
}

pub struct Executor {
    graph_factory: GraphFactory,
    graph_type: GraphType,
}

impl Executor {
    pub fn new(graph_factory: GraphFactory, graph_type: GraphType) -> Self {
        Executor {
            graph_factory,
            graph_type,
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<DataResult, String> {
        println!("Executing: {:?}", command);

        match &command.command_type {
            CommandType::CreateGraph(graph_name) => self
                .graph_factory
                .create_graph(graph_name.to_owned(), &self.graph_type),

            CommandType::ListGraphs => self.graph_factory.list_graphs(),

            CommandType::ListVertices(filter_command) => {
                let graph = self.get_graph(&command)?;
                graph.list_vertices(filter_command)
            }

            CommandType::GetVertex(id) => {
                let graph = self.get_graph(&command)?;
                graph.get_vertex(*id)
            }

            CommandType::AddVertex(label, mutate_command) => {
                let graph = self.get_mut_graph(&command)?;
                let vertex = create_vertex(label.to_string(), &mutate_command)?;
                graph.add_vertex(vertex)
            }

            CommandType::EditVertex(id, mutate_command) => {
                let graph = self.get_mut_graph(&command)?;
                let get_vertex_result = graph.get_mutable_vertex(*id)?;
                let vertex = match get_vertex_result {
                    DataResult::MutableVertexRef(vertex) => vertex,
                    _ => return Err(format!("Mismatched return type")),
                };

                let properties = update_vertex_properties(&mutate_command)?;

                vertex.update(properties)
            }

            CommandType::Help => Err(help()),
        }
    }

    fn get_mut_graph(&mut self, command: &Command) -> Result<&mut Box<dyn Graph>, String> {
        let graph_name = self.get_graph_name(command)?;

        self.graph_factory.get_graph(&graph_name)
    }

    fn get_graph(&mut self, command: &Command) -> Result<&Box<dyn Graph>, String> {
        let graph = self.get_mut_graph(command)?;
        Ok(&*graph)
    }

    fn get_graph_name(&self, command: &Command) -> Result<String, String> {
        let msg = "Graph not specified".to_string();

        match &command.command_json {
            Some(json) => {
                let name = json[JsonProperty::GraphName.as_str()].clone();
                match name.is_null() {
                    true => Err(msg),
                    false => Ok(name.to_string()),
                }
            }
            None => return Err(msg),
        }
    }
}

pub fn help() -> String {
    r#"
    
    Standalone Commands

        help: prints this page

        createGraph(<graph name>): creates a graph with the given name

        listGraphs(): lists all graphs

    Graph Commands (preceded with a graph name. E.g. graph.V()):

        .V(): lists vertices in the given graph
        
        .V(<id>): gets a vertex in the given graph

        .addV(<label>): adds a vertex to the given graph. This command can be used with no mutation commands to create an empty vertex

        .editV(<id>): selects a vertex with the given id for editing

    Vertex mutation commands (preceded with either addV() or editV(<id>))

        .property(<name>, <value>, <type>): adds a property to the given vertex

        .removeProperty(<name>): removes the property with the given name

    Vertex filter commands (preceded with V())

        .hasLabel(<label>): selects vertices with the given label

        .hasProperty(<name>): selects vertices with the given property name

        .hasPropertyValue(<name>, <value>): selects vertices with the given property name and value.
                
        .hasPropertyLike(<name>, <search_term>): selects vertices with properties matching the given search term (string property values only)

        .values(<name>): selects the value of the property with the given name for each selected vertex

    Vertex property types
        
        boolean
        int32
        int64
        float32
        float64
        string
        datetime (stored as ms since Unix epoch)
    "#
    .to_string()
}

fn create_vertex(
    label: String,
    mutate_command: &Vec<VertexMutationCommandType>,
) -> Result<Vertex, String> {
    let properties = update_vertex_properties(mutate_command)?;
    Ok(Vertex { label, properties })
}

fn update_vertex_properties(
    mutate_command: &Vec<VertexMutationCommandType>,
) -> Result<Vec<Property>, String> {
    let mut properties = Vec::new();
    for command in mutate_command {
        match command {
            VertexMutationCommandType::Property(property) => {
                properties.push(Property {
                    name: property.name.clone(),
                    value: property.value.clone(),
                    flagged_for_removal: false,
                });
            }
            VertexMutationCommandType::RemoveProperty(property_name) => {
                properties.push(Property {
                    name: property_name.clone(),
                    value: PropertyValue::Int32(0),
                    flagged_for_removal: true,
                });
            }
        }
    }

    Ok(properties)
}
