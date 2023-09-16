use crate::errors::error::Error;
use crate::types::{
    people::{Person, PersonID},
    pets::Pet,
};
use async_trait::async_trait;
use std::fmt::{Debug, Error as FmtError, Formatter};

#[async_trait]
pub trait Storer {
    async fn add_person(&self, new_person: Person) -> Result<Person, Error>;
    async fn get_person(&self, person_id: PersonID) -> Result<Person, Error>;
    async fn get_people(&self, limit: Option<i32>, offset: i32) -> Result<Vec<Person>, Error>;
    async fn update_person(&self, person: Person) -> Result<Person, Error>;
    async fn delete_person(&self, person_id: PersonID) -> Result<bool, Error>;
    async fn add_pet(&self, new_pet: Pet) -> Result<Pet, Error>;
}

impl Debug for dyn Storer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        f.debug_struct("Storer").finish()
    }
}
