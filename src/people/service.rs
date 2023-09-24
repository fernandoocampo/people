use crate::errors::error::Error;
use crate::people::storage;
use crate::types::{
    people::{NewPerson, Person, PersonID},
    pets::{NewPet, Pet},
};
use log::error;
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
            Err(e) => {
                error!("getting people from repository: {:?}", e);
                return Err(Error::GetPeopleError);
            }
        };

        res.sort();

        Ok(res)
    }

    pub async fn get_person(&self, person_id: PersonID) -> Result<Person, Error> {
        debug!("start getting person {}", person_id);

        match self.store.get_person(person_id).await {
            Ok(person) => Ok(person),
            Err(e) => {
                error!("getting person from repository: {:?}", e);
                Err(Error::GetPersonError)
            }
        }
    }

    pub async fn update_person(&self, person: Person) -> Result<Person, Error> {
        debug!("start updating person {}", person.id);
        match self.store.update_person(person).await {
            Ok(person) => Ok(person),
            Err(e) => {
                error!("updating person from repository: {:?}", e);
                Err(Error::UpdatePersonError)
            }
        }
    }

    pub async fn add_person(&self, new_person: NewPerson) -> Result<Person, Error> {
        debug!("start adding people {:?}", new_person);

        let person = new_person.to_person();
        debug!("new person with id {:?} is about to be saved", person);

        match self.store.add_person(person.clone()).await {
            Ok(person) => Ok(person),
            Err(e) => {
                error!("adding person into repository: {:?}", e);
                Err(Error::CreatePersonError)
            }
        }
    }

    pub async fn delete_person(&self, person_id: PersonID) -> Result<bool, Error> {
        debug!("start deleting person {}", person_id);

        match self.store.delete_person(person_id).await {
            Ok(ok) => Ok(ok),
            Err(e) => {
                error!("deleting person from repository: {:?}", e);
                Err(Error::DeletePersonError)
            }
        }
    }

    pub async fn add_pet(&self, new_pet: NewPet) -> Result<Pet, Error> {
        debug!("start adding pet {:?}", new_pet);

        let pet = new_pet.to_pet();

        match self.store.add_pet(pet.clone()).await {
            Ok(pet) => Ok(pet),
            Err(e) => {
                error!("adding pet into repository: {:?}", e);
                Err(Error::DeletePersonError)
            }
        }
    }
}
