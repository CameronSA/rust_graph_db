use json::JsonValue as Json;

#[derive(Debug)]
pub enum CommandType {
    CreateGraph(String),
    ListVertices,
    GetVertex(usize),
    AddVertex,
}

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub command_json: Option<Json>,
}

pub struct Executor;
impl Executor {
    pub fn execute(command: Command) {
        print!("{:?}", command);
    }
}
