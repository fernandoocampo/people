use std::{fmt, fmt::Display, fmt::Formatter, num::ParseIntError};
use warp::reject::Reject;
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    Rejection, Reply,
};

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(ParseIntError),
    MissingParameters,
    PersonNotFound,
    DatabaseQueryError,
    GetPeopleError,
    GetPersonError,
    CreatePersonError,
    UpdatePersonError,
    DeletePersonError,
    AddPetError,
    ValidateBadWordsError,
    ClientError(APILayerError),
    ServerError(APILayerError),
}

impl Reject for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {err}"),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::PersonNotFound => write!(f, "Person not found"),
            Error::DatabaseQueryError => write!(f, "Cannot update, invalid data"),
            Error::GetPeopleError => write!(f, "Unable to get people"),
            Error::GetPersonError => write!(f, "Unable to get person"),
            Error::CreatePersonError => write!(f, "Unable to create person"),
            Error::UpdatePersonError => write!(f, "Unable to update person"),
            Error::DeletePersonError => write!(f, "Unable to delete person"),
            Error::AddPetError => write!(f, "Unable to add pet"),
            Error::ValidateBadWordsError => write!(f, "cannot validate bad words"),
            Error::ClientError(ref err) => write!(f, "External Server error: {}", err),
            Error::ServerError(ref err) => write!(f, "External Server error: {}", err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::cmp::PartialEq for APILayerError {
    fn eq(&self, other: &Self) -> bool {
        if self.status != other.status {
            return false;
        }

        self.message == other.message
    }
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::GetPeopleError) = r.find() {
        Ok(warp::reply::with_status(
            "Cannot get people".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::GetPersonError) = r.find() {
        Ok(warp::reply::with_status(
            "Cannot get person".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::CreatePersonError) = r.find() {
        Ok(warp::reply::with_status(
            "Cannot create person".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::UpdatePersonError) = r.find() {
        Ok(warp::reply::with_status(
            "Cannot update person".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::DeletePersonError) = r.find() {
        Ok(warp::reply::with_status(
            "Cannot delete person".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::AddPetError) = r.find() {
        Ok(warp::reply::with_status(
            "Cannot add pet".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
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
    } else if let Some(Error::ValidateBadWordsError) = r.find() {
        Ok(warp::reply::with_status(
            "cannot validate bad words".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ClientError(_e)) = r.find() {
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ServerError(_e)) = r.find() {
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
