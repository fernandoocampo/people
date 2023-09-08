#[cfg(test)]
mod handler_tests {
    use crate::errors::error;
    use crate::people::handler;
    use crate::storage::memory::Store;
    use crate::types::people::{Person, PersonID};
    use std::collections::HashMap;
    use tokio::runtime::Runtime;
    use warp::{Rejection, Reply};

    #[test]
    fn test_get_people() {
        // Given
        let store = Store::new();
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("start"), String::from("0"));
        params.insert(String::from("end"), String::from("2"));

        let expected_people: Vec<Person> = vec![
            Person {
                id: PersonID("1".to_string()),
                name: "Luis".to_string(),
            },
            Person {
                id: PersonID("2".to_string()),
                name: "Fernando".to_string(),
            },
        ];

        let want = new_people_result(expected_people, None);

        let runtime = Runtime::new().expect("Unable to create a runtime");
        // When
        let got = runtime.block_on(handler::get_people(params, store));
        // Then
        assert_eq!(got.is_err(), want.is_err());
        // assert_eq!(got.unwrap_err(), expected_result.unwrap_err());
        let got_response = got.unwrap().into_response();
        let want_response = want.unwrap().into_response();
        assert_eq!(got_response.status(), want_response.status());

        let got_body_bytes = runtime
            .block_on(hyper::body::to_bytes(got_response.into_body()))
            .unwrap();
        let got_body_string = String::from_utf8(got_body_bytes.to_vec()).unwrap();

        let want_body_bytes = runtime
            .block_on(hyper::body::to_bytes(want_response.into_body()))
            .unwrap();
        let want_body_string = String::from_utf8(want_body_bytes.to_vec()).unwrap();

        assert_eq!(got_body_string, want_body_string);
    }

    #[test]
    fn test_get_person() {
        // Given
        let store = Store::new();
        let person_id = "1".to_string();
        let expected_person = Person {
            id: PersonID("1".to_string()),
            name: "Luis".to_string(),
        };
        let runtime = Runtime::new().expect("unable to create runtime to test get person");
        // When
        let response = runtime.block_on(handler::get_person(person_id, store));
        // Then
        let got_body_bytes = runtime
            .block_on(hyper::body::to_bytes(
                response.unwrap().into_response().into_body(),
            ))
            .unwrap();
        let got: Person = serde_json::from_slice(&got_body_bytes).unwrap();
        assert_eq!(expected_person, got);
    }

    #[test]
    fn test_get_not_found_person() {
        // Given
        let store = Store::new();
        let person_id = "2000".to_string();
        let runtime = Runtime::new().expect("unable to create runtime to test get person");
        // When
        let got = runtime.block_on(handler::get_person(person_id, store));
        // Then
        assert_eq!(true, got.is_err());

        let got_error = match got {
            Ok(_) => panic!("unexpected value"),
            Err(err) => err,
        };

        if let Some(e) = got_error.find::<error::Error>() {
            assert_eq!(*e, error::Error::PersonNotFound);
            return;
        }
    }

    fn new_people_result(
        people: Vec<Person>,
        err: Option<error::Error>,
    ) -> Result<impl Reply, Rejection> {
        if err.is_some() {
            return Err(warp::reject::custom(err.unwrap()));
        }

        Ok(warp::reply::json(&people))
    }
}
