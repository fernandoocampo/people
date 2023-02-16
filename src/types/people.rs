use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person {
    pub id: PersonID,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PersonID(pub String);

impl Person {
    pub fn new(id: PersonID, name: String) -> Self {
        Person { id, name }
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
