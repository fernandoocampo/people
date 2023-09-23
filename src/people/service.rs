use crate::errors::error::Error;
use crate::people::storage;
use crate::types::{
    people::{NewPerson, Person, PersonID},
    pets::{NewPet, Pet},
};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct Service<T: storage::Storer> {
    store: T,
}

impl<T: storage::Storer> Service<T> {
    pub fn new(a_store: T) -> Self {
        Service { store: a_store }
    }

    pub async fn get_people(&self, limit: Option<i32>, offset: i32) -> Result<Vec<Person>, Error> {
        debug!("start querying people");

        let mut res = match self.store.get_people(limit, offset).await {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        res.sort();

        Ok(res)
    }

    pub async fn get_person(&self, person_id: PersonID) -> Result<Person, Error> {
        self.store.get_person(person_id).await
    }

    pub async fn update_person(&self, person: Person) -> Result<Person, Error> {
        self.store.update_person(person).await
    }

    pub async fn add_person(&self, new_person: NewPerson) -> Result<Person, Error> {
        debug!("start adding people {:?}", new_person);

        let person = new_person.to_person();
        debug!("new person with id {:?} is about to be saved", person);

        self.store.add_person(person.clone()).await
    }

    pub async fn delete_person(&self, person_id: PersonID) -> Result<bool, Error> {
        self.store.delete_person(person_id).await
    }

    pub async fn add_pet(&self, new_pet: NewPet) -> Result<Pet, Error> {
        let pet = new_pet.to_pet();

        self.store.add_pet(pet.clone()).await
    }
}
