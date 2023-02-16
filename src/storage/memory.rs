use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::{
    people::{Person, PersonID},
    pets::{Pet, PetID},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub people: Arc<RwLock<HashMap<PersonID, Person>>>,
    pub pets: Arc<RwLock<HashMap<PetID, Pet>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            people: Arc::new(RwLock::new(Self::init())),
            pets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<PersonID, Person> {
        let file = include_str!("../../people.json");
        serde_json::from_str(file).expect("can't read people.json")
    }
}

impl Default for Store {
    fn default() -> Self {
        Store::new()
    }
}
