use crate::errors::error::Error;
use crate::people::storage;
use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use tracing::debug;

use crate::types::{
    people::{Person, PersonID},
    pets::{Pet, PetID},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("couldn't establish DB connection! {e}"),
        };

        Store {
            connection: db_pool,
        }
    }
}

#[async_trait]
impl storage::Storer for Store {
    async fn get_people(&self, limit: Option<i32>, offset: i32) -> Result<Vec<Person>, Error> {
        match sqlx::query("SELECT * FROM people LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Person {
                id: PersonID(row.get("id")),
                name: row.get("name"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(people) => {
                debug!("found some people: {:?}", people);
                Ok(people)
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    async fn get_person(&self, person_id: PersonID) -> Result<Person, Error> {
        match sqlx::query("SELECT * FROM people WHERE ID = $1")
            .bind(person_id.to_string())
            .map(|row: PgRow| Person {
                id: PersonID(row.get("id")),
                name: row.get("name"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(person) => Ok(person),
            Err(sqlx::Error::RowNotFound) => {
                Ok(Person::new(PersonID(String::from("")), String::from("")))
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    async fn add_person(&self, new_person: Person) -> Result<Person, Error> {
        debug!("adding person to postgres database: {:?}", new_person);

        match sqlx::query("INSERT INTO people (ID, NAME) VALUES ($1, $2) RETURNING ID, NAME")
            .bind(new_person.id.to_string())
            .bind(new_person.name)
            .map(|row: PgRow| Person {
                id: PersonID(row.get("id")),
                name: row.get("name"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(person) => {
                debug!("person was added to postgres database: {:?}", person);
                Ok(person)
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    async fn update_person(&self, person: Person) -> Result<Person, Error> {
        match sqlx::query("UPDATE people SET NAME=$1 WHERE ID=$2 RETURNING ID, NAME")
            .bind(person.name)
            .bind(person.id.to_string())
            .map(|row: PgRow| Person {
                id: PersonID(row.get("id")),
                name: row.get("name"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(person) => Ok(person),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    async fn delete_person(&self, person_id: PersonID) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM people WHERE id = $1")
            .bind(person_id.to_string())
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    async fn add_pet(&self, new_pet: Pet) -> Result<Pet, Error> {
        match sqlx::query("INSERT INTO pets (ID, NAME, PERSON_ID) VALUES ($1, $2, $3) RETURNING ID, NAME, PERSON_ID")
            .bind(new_pet.id.to_string())
            .bind(new_pet.name)
            .bind(new_pet.person_id.to_string())
            .map(|row: PgRow| Pet{
                id: PetID(row.get("id")),
                name: row.get("name"),
                person_id: PersonID(row.get("PERSON_ID")),
            })
            .fetch_one(&self.connection)
            .await{
                Ok(pet) => Ok(pet),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "{:?}", e);
                    Err(Error::DatabaseQueryError)
                }
            }
    }
}

// impl Default for Store {
//     fn default() -> Self {
//         Store::new()
//     }
// }
