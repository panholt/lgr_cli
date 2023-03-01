pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self { id: 1,
               name: name,
               description: description,
               status: Status::Open,
               stories:  Vec::new(),
        }
    }
}

pub struct Story {
    pub id: u32,
    pub name: String,
    pub description: String, 
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self { id: 1, 
               name: name,
               description: description, 
               status: Status::Open }
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
}
