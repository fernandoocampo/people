#[cfg(test)]
mod handler_tests {
    use crate::errors::error;
    use crate::people::{handler, service, storage};
    use crate::types::people::{NewPerson, Person, PersonID, SavePersonSuccess};
    use crate::types::pets::Pet;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use tokio::runtime::Runtime;
    use warp::{http::StatusCode, Rejection, Reply};

    #[test]
    fn test_get_people() {
        // Given
        let people_store = vec![
            Person {
                id: PersonID("1".to_string()),
                name: "Luis".to_string(),
            },
            Person {
                id: PersonID("2".to_string()),
                name: "Fernando".to_string(),
            },
        ];

        let person_service =
            service::Service::new(DummyStore::new_with_get_people(people_store, false));

        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("offset"), String::from("0"));
        params.insert(String::from("limit"), String::from("2"));

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
        let got = runtime.block_on(handler::get_people(params, person_service));
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
        let person_store = Person {
            id: PersonID("1".to_string()),
            name: "Luis".to_string(),
        };
        let person_service =
            service::Service::new(DummyStore::new_with_get_person(Some(person_store), false));
        let person_id = "1".to_string();
        let expected_person = Person {
            id: PersonID("1".to_string()),
            name: "Luis".to_string(),
        };
        let runtime = Runtime::new().expect("unable to create runtime to test get person");
        // When
        let response = runtime.block_on(handler::get_person(person_id, person_service));
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
        let person_service = service::Service::new(DummyStore::new_with_get_person(None, true));
        let person_id = "2000".to_string();
        let runtime = Runtime::new().expect("unable to create runtime to test get person");
        // When
        let got = runtime.block_on(handler::get_person(person_id, person_service));
        // Then
        assert_eq!(true, got.is_err());

        let got_error = match got {
            Ok(value) => panic!("unexpected result {:?}", value.into_response()),
            Err(err) => err,
        };

        if let Some(e) = got_error.find::<error::Error>() {
            assert_eq!(*e, error::Error::DatabaseQueryError);
            return;
        }
    }

    #[test]
    fn test_create_person() {
        // Given
        let person = Person {
            id: PersonID("3".to_string()),
            name: "esme".to_string(),
        };
        let new_person = NewPerson::new("esme".to_string());
        let mut expected_result = SavePersonSuccess { id: "".to_string() };
        let person_service =
            service::Service::new(DummyStore::new_with_add_person(Some(person), false));
        let runtime = Runtime::new().expect("unable to create runtime to test create person");
        // When
        let got = runtime.block_on(handler::add_person(new_person, person_service));
        // Then
        assert_eq!(false, got.is_err());

        let got_result = match got {
            Ok(reply) => {
                let reply_response = reply.into_response();
                assert_eq!(StatusCode::OK, reply_response.status());
                let result = runtime
                    .block_on(hyper::body::to_bytes(reply_response.into_body()))
                    .unwrap();
                let response = std::str::from_utf8(&result).unwrap();
                response.to_string()
            }
            Err(err) => panic!("unexpected error: {:?}", err),
        };

        let got_person_id: SavePersonSuccess = serde_json::from_str(got_result.as_str()).unwrap();
        expected_result.id = got_person_id.id.clone();

        assert_eq!(got_person_id, expected_result);
    }

    #[test]
    fn test_delete_person() {
        // Given
        let person_service = service::Service::new(DummyStore::new_with_delete_person(false));
        let person_id = "2".to_string();
        let expected_result = "Person 2 deleted";
        let runtime = Runtime::new().expect("unable to create runtime to test delete person");
        // When
        let got = runtime.block_on(handler::delete_person(person_id, person_service));
        // Then
        assert_eq!(false, got.is_err());

        let got_result = match got {
            Ok(reply) => {
                let reply_response = reply.into_response();
                assert_eq!(StatusCode::OK, reply_response.status());
                let result = runtime
                    .block_on(hyper::body::to_bytes(reply_response.into_body()))
                    .unwrap();
                let response = std::str::from_utf8(&result).unwrap();
                response.to_string()
            }
            Err(err) => panic!("unexpected error: {:?}", err),
        };

        assert_eq!(got_result, expected_result);
    }

    #[test]
    fn test_delete_person_but_not_found() {
        // Given
        let person_service = service::Service::new(DummyStore::new_with_delete_person(true));
        let person_id = "2000".to_string();
        let runtime = Runtime::new().expect("unable to create runtime to test delete person");
        // When
        let got = runtime.block_on(handler::delete_person(person_id, person_service));
        // Then
        assert_eq!(true, got.is_err());

        let got_error = match got {
            Ok(value) => panic!("unexpected result {:?}", value.into_response()),
            Err(err) => err,
        };

        if let Some(e) = got_error.find::<error::Error>() {
            assert_eq!(*e, error::Error::DatabaseQueryError);
            return;
        }
    }

    #[test]
    fn test_update_person() {
        // Given
        let a_person = Person {
            id: PersonID("1".to_string()),
            name: "Luisfer".to_string(),
        };
        let person_to_return = Some(Person {
            id: PersonID("1".to_string()),
            name: "Luisfer".to_string(),
        });
        let person_service =
            service::Service::new(DummyStore::new_with_update_person(person_to_return, false));
        let expected_result = Person {
            id: PersonID("1".to_string()),
            name: "Luisfer".to_string(),
        };
        let runtime = Runtime::new().expect("unable to create runtime to test update person");
        // When
        let got = runtime.block_on(handler::update_person(a_person, person_service));
        // Then
        assert_eq!(false, got.is_err());

        let got_result = match got {
            Ok(reply) => {
                let reply_response = reply.into_response();
                assert_eq!(StatusCode::OK, reply_response.status());
                let result = runtime
                    .block_on(hyper::body::to_bytes(reply_response.into_body()))
                    .unwrap();
                let response = std::str::from_utf8(&result).unwrap();
                response.to_string()
            }
            Err(err) => panic!("unexpected value: {:?}", err),
        };

        let got_person: Person = serde_json::from_str(got_result.as_str()).unwrap();

        assert_eq!(got_person, expected_result);
    }

    #[test]
    fn test_update_person_but_not_found() {
        // Given
        let a_person = Person {
            id: PersonID("1".to_string()),
            name: "Luisfer".to_string(),
        };
        let person_service = service::Service::new(DummyStore::new_with_update_person(None, true));
        let runtime = Runtime::new().expect("unable to create runtime to test update person");
        // When
        let got = runtime.block_on(handler::update_person(a_person, person_service));
        // Then
        assert_eq!(true, got.is_err());

        let got_error = match got {
            Ok(value) => panic!("unexpected result {:?}", value.into_response()),
            Err(err) => err,
        };

        if let Some(e) = got_error.find::<error::Error>() {
            assert_eq!(*e, error::Error::DatabaseQueryError);
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

    #[derive(Debug, Clone)]
    struct DummyStore {
        get_people_values: Option<Vec<Person>>,
        get_people_error: Option<bool>,
        get_person_value: Option<Person>,
        get_person_error: Option<bool>,
        add_person_value: Option<Person>,
        add_person_error: Option<bool>,
        update_person_value: Option<Person>,
        update_person_error: Option<bool>,
        delete_person_value: Option<bool>,
        delete_person_error: Option<bool>,
        add_pet_value: Option<Pet>,
        add_pet_error: Option<bool>,
    }

    impl DummyStore {
        fn new_with_get_people(people: Vec<Person>, is_error: bool) -> Self {
            let mut dummy_store = DummyStore::default();
            dummy_store.get_people_values = Some(people);
            dummy_store.get_people_error = Some(is_error);

            dummy_store
        }
        fn new_with_get_person(person: Option<Person>, is_error: bool) -> Self {
            let mut dummy_store = DummyStore::default();
            dummy_store.get_person_value = person;
            dummy_store.get_person_error = Some(is_error);

            dummy_store
        }
        fn new_with_add_person(person: Option<Person>, is_error: bool) -> Self {
            let mut dummy_store = DummyStore::default();
            dummy_store.add_person_value = person;
            dummy_store.add_person_error = Some(is_error);

            dummy_store
        }
        fn new_with_delete_person(is_error: bool) -> Self {
            let mut dummy_store = DummyStore::default();
            dummy_store.delete_person_error = Some(is_error);
            dummy_store.delete_person_value = Some(is_error);

            dummy_store
        }
        fn new_with_update_person(person: Option<Person>, is_error: bool) -> Self {
            let mut dummy_store = DummyStore::default();
            dummy_store.update_person_error = Some(is_error);
            dummy_store.update_person_value = person;

            dummy_store
        }
    }

    impl Default for DummyStore {
        fn default() -> DummyStore {
            DummyStore {
                get_people_values: Default::default(),
                get_people_error: Default::default(),
                get_person_value: Default::default(),
                get_person_error: Default::default(),
                add_person_value: Default::default(),
                add_person_error: Default::default(),
                update_person_value: Default::default(),
                update_person_error: Default::default(),
                delete_person_value: Default::default(),
                delete_person_error: Default::default(),
                add_pet_value: Default::default(),
                add_pet_error: Default::default(),
            }
        }
    }

    #[async_trait]
    impl storage::Storer for DummyStore {
        async fn get_people(&self, _: Option<i32>, _: i32) -> Result<Vec<Person>, error::Error> {
            match self.get_people_error.unwrap() {
                false => Ok(self.get_people_values.clone().unwrap()),
                true => Err(error::Error::DatabaseQueryError),
            }
        }

        async fn get_person(&self, _: PersonID) -> Result<Person, error::Error> {
            match &self.get_person_error.unwrap() {
                false => Ok(self.get_person_value.clone().unwrap()),
                true => Err(error::Error::DatabaseQueryError),
            }
        }

        async fn add_person(&self, _: Person) -> Result<Person, error::Error> {
            match &self.add_person_error.unwrap() {
                false => Ok(self.add_person_value.clone().unwrap()),
                true => Err(error::Error::DatabaseQueryError),
            }
        }

        async fn update_person(&self, _: Person) -> Result<Person, error::Error> {
            match &self.update_person_error.unwrap() {
                false => Ok(self.update_person_value.clone().unwrap()),
                true => Err(error::Error::DatabaseQueryError),
            }
        }

        async fn delete_person(&self, _: PersonID) -> Result<bool, error::Error> {
            match &self.delete_person_error.unwrap() {
                false => Ok(self.delete_person_value.unwrap()),
                true => Err(error::Error::DatabaseQueryError),
            }
        }

        async fn add_pet(&self, _: Pet) -> Result<Pet, error::Error> {
            match &self.add_pet_error.unwrap() {
                false => Ok(self.add_pet_value.clone().unwrap()),
                true => Err(error::Error::DatabaseQueryError),
            }
        }
    }
}
