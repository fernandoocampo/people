use serde::{Deserialize, Serialize};
use std::fmt;

use crate::types::people::PersonID;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PetID(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Pet {
    pub id: PetID,
    pub name: String,
    pub person_id: PersonID,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewPet {
    pub name: String,
    pub person_id: PersonID,
}

impl NewPet {
    pub fn new(name: String, person_id: PersonID) -> Self {
        NewPet { name, person_id }
    }

    pub fn to_pet(&self) -> Pet {
        Pet {
            id: PetID(uuid::Uuid::new_v4().to_string()),
            name: self.name.clone(),
            person_id: self.person_id.clone(),
        }
    }
}

impl fmt::Display for PetID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
