use serde::{Deserialize, Serialize};
use std::{
    fmt,
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person {
    pub id: PersonID,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewPerson {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SavePersonSuccess {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PersonID(pub String);

impl Person {
    pub fn new(id: PersonID, name: String) -> Self {
        Person { id, name }
    }
}

impl NewPerson {
    pub fn new(name: String) -> Self {
        NewPerson { name }
    }

    pub fn to_person(&self) -> Person {
        Person {
            id: PersonID(uuid::Uuid::new_v4().to_string()),
            name: self.name.clone(),
        }
    }
}

impl SavePersonSuccess {
    pub fn new(person_id: PersonID) -> Self {
        SavePersonSuccess {
            id: person_id.to_string(),
        }
    }
}

impl FromStr for PersonID {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(PersonID(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

impl fmt::Display for PersonID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
