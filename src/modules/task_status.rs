use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]

pub enum Status{
    Active,
    Inactive,
    InProgress,
    Completed,
}