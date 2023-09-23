use crate::people::{service, storage};
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
    service: service::Service<impl storage::Storer>,
) -> Result<impl Reply, Rejection> {
    debug!("start querying people");

    let mut pagination = pagination::Pagination::default();
    if !params.is_empty() {
        debug!(pagination = false);
        pagination = pagination::extract_pagination(params)?;
    }

    debug!(pagination = true);
    let mut res: Vec<Person> = match service
        .get_people(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    res.sort();

    Ok(warp::reply::json(&res))
}

pub async fn get_person(
    id: String,
    service: service::Service<impl storage::Storer>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match service.get_person(PersonID(id)).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn update_person(
    person: Person,
    service: service::Service<impl storage::Storer>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match service.update_person(person).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_person(
    new_person: NewPerson,
    service: service::Service<impl storage::Storer>,
) -> Result<impl warp::Reply, warp::Rejection> {
    debug!("start adding people {:?}", new_person);

    match service.add_person(new_person.clone()).await {
        Ok(person) => {
            debug!("new person was saved {:?}", person);

            let result = SavePersonSuccess::new(person.id);

            Ok(warp::reply::json(&result))
        }
        Err(e) => {
            error!("adding person {:?}", new_person);
            Err(warp::reject::custom(e))
        }
    }
}

pub async fn delete_person(
    id: String,
    service: service::Service<impl storage::Storer>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = service.delete_person(PersonID(id.clone())).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status(
        format!("Person {} deleted", id),
        StatusCode::OK,
    ))
}

pub async fn add_pet(
    new_pet: NewPet,
    service: service::Service<impl storage::Storer>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match service.add_pet(new_pet.clone()).await {
        Ok(pet) => Ok(warp::reply::with_status(pet.id.to_string(), StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
