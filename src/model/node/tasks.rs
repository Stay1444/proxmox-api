use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct GetTasksFilter {
    #[serde(rename = "errors")]
    pub only_errors: Option<bool>,
    pub limit: Option<u64>,
    pub since: Option<u64>,
    pub source: Option<TaskSource>,
    pub start: Option<u64>,
    #[serde(
        rename = "statusfilter",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub status_filter: Option<String>,
    #[serde(
        rename = "typefilter",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub type_filter: Option<String>,
    pub until: Option<u64>,
    #[serde(
        rename = "userfilter",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub user_filter: Option<String>,
    pub vmid: Option<u64>,
}

impl Default for GetTasksFilter {
    fn default() -> Self {
        Self {
            only_errors: Some(false),
            limit: Some(50),
            since: None,
            source: Some(TaskSource::Archive),
            start: Some(0),
            status_filter: None,
            type_filter: None,
            until: None,
            user_filter: None,
            vmid: None,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TaskSource {
    Archive,
    Active,
    All,
}

#[derive(Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub node: String,
    pub pid: u64,
    pub pstart: u64,
    #[serde(rename = "starttime")]
    pub start_time: u64,
    #[serde(rename = "type")]
    pub task_type: String,
    pub upid: String,
    pub user: String,
    #[serde(default)]
    pub end_time: Option<u64>,
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TaskStatus {
    pub id: String,
    pub node: String,
    pub pid: u64,
    #[serde(rename = "starttime")]
    pub start_time: u64,
    pub status: TaskState,
    #[serde(rename = "type")]
    pub task_type: String,
    pub upid: String,
    pub user: String,
    #[serde(
        default,
        rename = "exitstatus",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub exit_status: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TaskState {
    Running,
    Stopped,
}
