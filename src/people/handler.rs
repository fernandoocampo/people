use crate::errors::error;
use crate::storage::memory::Store;
use crate::types::{
    pagination,
    people::{Person, PersonID},
    pets::{Pet, PetID},
};
use std::collections::HashMap;
use warp::{hyper::StatusCode, reject::Reject, Rejection, Reply};

#[derive(Debug)]
struct InvalidID;
impl Reject for InvalidID {}

pub async fn get_people(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if params.is_empty() {
        let res: Vec<Person> = store.people.read().await.values().cloned().collect();
        return Ok(warp::reply::json(&res));
    }

    let pagination = pagination::extract_pagination(params)?;
    let mut res: Vec<Person> = store.people.read().await.values().cloned().collect();
    res.sort();
    let res: &[Person] = &res[pagination.start..pagination.end];

    Ok(warp::reply::json(&res))
}

pub async fn get_person(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.people.write().await.get_mut(&PersonID(id)) {
        Some(p) => Ok(warp::reply::json(&p)),
        None => Err(warp::reject::custom(error::Error::PersonNotFound)),
    }
}

pub async fn update_person(
    id: String,
    person: Person,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.people.write().await.get_mut(&PersonID(id)) {
        Some(p) => *p = person,
        None => return Err(warp::reject::custom(error::Error::PersonNotFound)),
    }

    Ok(warp::reply::with_status("Person updated", StatusCode::OK))
}

pub async fn add_person(store: Store, person: Person) -> Result<impl warp::Reply, warp::Rejection> {
    store.people.write().await.insert(person.id.clone(), person);

    Ok(warp::reply::with_status("People added", StatusCode::OK))
}

pub async fn delete_person(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.people.write().await.remove(&PersonID(id)) {
        Some(_) => Ok(warp::reply::with_status("Person deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(error::Error::PersonNotFound)),
    }
}

pub async fn add_pet(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let pet = Pet {
        id: PetID("1".to_string()),
        name: params.get("name").unwrap().to_string(),
        person_id: PersonID(params.get("personID").unwrap().to_string()),
    };

    store.pets.write().await.insert(pet.id.clone(), pet);

    Ok(warp::reply::with_status("Pet Added", StatusCode::OK))
}
