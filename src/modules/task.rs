use crate::modules::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Tasks{
    pub task_id: i32,
    pub task_name: String,
    pub task_description: String,
    pub task_status: Status,
    pub date_created: String,
    pub date_due: String,
}