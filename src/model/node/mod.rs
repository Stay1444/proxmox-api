use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub localtime: i64,   // Seconds since 1970-01-01 00:00:00 (local time)
    pub time: i64,        // Seconds since 1970-01-01 00:00:00 UTC.
    pub timezone: String, // Time zone
}

pub struct NodeId(pub(crate) String);

impl NodeId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for NodeId {
    fn from(val: &str) -> Self {
        NodeId(val.into())
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

pub struct VMId(pub(crate) String);

impl VMId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for VMId {
    fn from(val: &str) -> Self {
        VMId(val.into())
    }
}

impl std::fmt::Display for VMId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
