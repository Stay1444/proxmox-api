use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TasksFilter {
    #[serde(rename = "errors")]
    pub only_errors: Option<bool>, // Only list tasks with a status of ERROR.
    pub limit: Option<u64>,         // Only list this amount of tasks.
    pub since: Option<u64>,         // Only list tasks since this UNIX epoch.
    pub source: Option<TaskSource>, // List archived, active or all tasks.
    pub start: Option<u64>,         // List tasks beginning from this offset.
    #[serde(
        rename = "statusfilter",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub status_filter: Option<String>, // List of Task States that should be returned.
    #[serde(
        rename = "typefilter",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub type_filter: Option<String>, // Only list tasks of this type (e.g., vzstart, vzdump).
    pub until: Option<u64>,         // Only list tasks until this UNIX epoch.
    #[serde(
        rename = "userfilter",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub user_filter: Option<String>, // Only list tasks from this user.
    pub vmid: Option<u64>,          // Only list tasks for this VM.
}

impl Default for TasksFilter {
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
