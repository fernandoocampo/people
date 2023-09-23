use crate::people::storage;
use crate::types::{
    pagination,
    people::{NewPerson, Person, PersonID, SavePersonSuccess},
    pets::NewPet,
};
use std::collections::HashMap;
use tracing::{debug, error};
use warp::{http::StatusCode, reject::Reject, Rejection, Reply};

#[derive(Debug)]
struct InvalidID;
impl Reject for InvalidID {}

pub async fn get_people(
    params: HashMap<String, String>,
    store: impl storage::Storer,
) -> Result<impl Reply, Rejection> {
    debug!("start querying people");

    let mut pagination = pagination::Pagination::default();
    if !params.is_empty() {
        debug!(pagination = false);
        pagination = pagination::extract_pagination(params)?;
    }

    debug!(pagination = true);
    let mut res: Vec<Person> = match store.get_people(pagination.limit, pagination.offset).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    res.sort();

    Ok(warp::reply::json(&res))
}

pub async fn get_person(
    id: String,
    store: impl storage::Storer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.get_person(PersonID(id)).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn update_person(
    person: Person,
    store: impl storage::Storer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_person(person).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_person(
    store: impl storage::Storer,
    new_person: NewPerson,
) -> Result<impl warp::Reply, warp::Rejection> {
    debug!("start adding people {:?}", new_person);

    let person = new_person.to_person();
    debug!("new person with id {:?} is about to be saved", person);

    if let Err(e) = store.add_person(person.clone()).await {
        error!("adding person {:?}", person);
        return Err(warp::reject::custom(e));
    }

    debug!("new person was saved {:?}", person);

    let result = SavePersonSuccess::new(person.id);

    Ok(warp::reply::json(&result))
}

pub async fn delete_person(
    id: String,
    store: impl storage::Storer,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_person(PersonID(id.clone())).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status(
        format!("Person {} deleted", id),
        StatusCode::OK,
    ))
}

pub async fn add_pet(
    store: impl storage::Storer,
    new_pet: NewPet,
) -> Result<impl warp::Reply, warp::Rejection> {
    let pet = new_pet.to_pet();

    if let Err(e) = store.add_pet(pet.clone()).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status(pet.id.to_string(), StatusCode::OK))
}
