use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,  // Make the id optional
    pub name: String,
    pub description: String,
}

impl Item {
    // A function to create a new item
    pub fn new(name: String, description: String) -> Self {
        Item {
            id: Some(Uuid::new_v4()), // Generate an id here
            name,
            description,
        }
    }
}
