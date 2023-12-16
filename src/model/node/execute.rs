use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Command {
    pub args: String,          // A set of parameter names and their values.
    pub method: CommandMethod, // A method related to the API endpoint (GET, POST etc.).
    pub path: String,          // A relative path to an API endpoint on this node.
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum CommandMethod {
    Get,
    Post,
    Put,
    Delete,
}
