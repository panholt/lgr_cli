use std:: collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Epic {
    // pub id: i32,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<i32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {name: name,
               description: description,
               status: Status::Open,
               stories:  Vec::new(),
        }
    }
}
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Story {
    // pub id: i32,
    pub name: String,
    pub description: String, 
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self { name: name,
               description: description, 
               status: Status::Open }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    pub last_item_id: i32,
    pub stories: HashMap<i32, Story>,
    pub epics: HashMap<i32, Epic>,
}
