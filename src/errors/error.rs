use std::{fmt, fmt::Display, fmt::Formatter, num::ParseIntError};
use warp::reject::Reject;
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    hyper::StatusCode,
    Rejection, Reply,
};

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(ParseIntError),
    MissingParameters,
    PersonNotFound,
}

impl Reject for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {err}"),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::PersonNotFound => write!(f, "Person not found"),
        }
    }
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
