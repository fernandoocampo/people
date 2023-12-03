use crate::errors::error::Error;
use crate::people::storage::Storer as people_storage;
use crate::users::storage::Storer as users_storage;
use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use tracing::debug;

use crate::types::{
    accounts::{Account, AccountID},
    people::{Person, PersonID},
    pets::{Pet, PetID},
};

const DUPLICATE_KEY: i32 = 23505;

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
impl people_storage for Store {
    async fn get_people(&self, limit: Option<i32>, offset: i32) -> Result<Vec<Person>, Error> {
        match sqlx::query("SELECT * FROM people LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Person {
                id: PersonID(row.get("id")),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
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
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(person) => Ok(person),
            Err(sqlx::Error::RowNotFound) => Ok(Person::new(
                PersonID(String::from("")),
                String::from(""),
                String::from(""),
            )),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    async fn add_person(&self, new_person: Person) -> Result<Person, Error> {
        debug!("adding person to postgres database: {:?}", new_person);

        match sqlx::query("INSERT INTO people (ID, FIRST_NAME, LAST_NAME) VALUES ($1, $2, $3) RETURNING ID, FIRST_NAME, LAST_NAME")
            .bind(new_person.id.to_string())
            .bind(new_person.first_name)
            .bind(new_person.last_name)
            .map(|row: PgRow| Person {
                id: PersonID(row.get("id")),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
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
        match sqlx::query("UPDATE people SET FIRST_NAME=$1, LAST_NAME=$2 WHERE ID=$3 RETURNING ID, FIRST_NAME, LAST_NAME")
            .bind(person.first_name)
            .bind(person.last_name)
            .bind(person.id.to_string())
            .map(|row: PgRow| Person {
                id: PersonID(row.get("id")),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
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

#[async_trait]
impl users_storage for Store {
    async fn add_account(&self, new_account: Account) -> Result<AccountID, Error> {
        debug!("adding account to postgres database: {}", new_account.email);

        match sqlx::query(
            "INSERT INTO accounts (ID, EMAIL, PASSWORD) VALUES ($1, $2, $3) RETURNING ID",
        )
        .bind(new_account.id.to_string())
        .bind(new_account.email)
        .bind(new_account.password)
        .map(|row: PgRow| AccountID(row.get("id")))
        .fetch_one(&self.connection)
        .await
        {
            Ok(account_id) => {
                debug!("new account was added to postgres database: {}", account_id);
                Ok(account_id)
            }
            Err(e) => {
                if get_sql_code(&e) == DUPLICATE_KEY {
                    tracing::event!(tracing::Level::INFO, message = "account already exists");

                    return Err(Error::DatabaseUniqueError);
                }

                tracing::event!(
                    tracing::Level::ERROR,
                    code = e
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = e.as_database_error().unwrap().message(),
                    constraint = e.as_database_error().unwrap().constraint().unwrap()
                );
                Err(Error::DatabaseQueryError)
            }
        }
    }

    async fn get_account(&self, email: String) -> Result<Account, Error> {
        debug!("getting account from postgres database: {}", email);

        match sqlx::query("SELECT * FROM accounts WHERE email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: AccountID(row.get("id")),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(sqlx::Error::RowNotFound) => Ok(Account::default()),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
}

fn get_sql_code(err: &sqlx::error::Error) -> i32 {
    err.as_database_error()
        .unwrap()
        .code()
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

// impl Default for Store {
//     fn default() -> Self {
//         Store::new()
//     }
// }
