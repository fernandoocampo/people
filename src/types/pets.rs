use serde::{Deserialize, Serialize};

use crate::types::people::PersonID;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PetID(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Pet {
    pub id: PetID,
    pub name: String,
    pub person_id: PersonID,
}
