use crate::errors::error;
use crate::people::{censor, service, storage};
use crate::types::people::{NewPerson, Person, PersonID};
use crate::types::pets::Pet;
use async_trait::async_trait;
use tokio::runtime::Runtime;

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

    let want: Vec<Person> = vec![
        Person {
            id: PersonID("1".to_string()),
            name: "Luis".to_string(),
        },
        Person {
            id: PersonID("2".to_string()),
            name: "Fernando".to_string(),
        },
    ];

    let a_store = DummyStore::new_with_get_people(people_store, false);
    let a_censor = DummyCensor::new("".to_string(), false);

    let person_service = service::Service::new(a_store, a_censor);

    let limit = Some(10);
    let offset = 0;

    let runtime = Runtime::new().expect("Unable to create a runtime");

    // When
    let got = runtime.block_on(person_service.get_people(limit, offset));

    // Then
    match got {
        Ok(people_got) => assert_eq!(want, people_got),
        Err(err) => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn test_get_person() {
    // Given
    let person_store = Person {
        id: PersonID("1".to_string()),
        name: "Luis".to_string(),
    };

    let a_store = DummyStore::new_with_get_person(Some(person_store), false);
    let a_censor = DummyCensor::new("".to_string(), false);
    let person_service = service::Service::new(a_store, a_censor);
    let person_id = PersonID("1".to_string());
    let want = Person {
        id: PersonID("1".to_string()),
        name: "Luis".to_string(),
    };
    let runtime = Runtime::new().expect("unable to create runtime to test get person");
    // When
    let got = runtime.block_on(person_service.get_person(person_id));
    // Then
    match got {
        Ok(person_got) => assert_eq!(want, person_got),
        Err(err) => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn test_get_not_found_person() {
    // Given
    let person_service = service::Service::new(
        DummyStore::new_with_get_person(None, true),
        DummyCensor::new("".to_string(), false),
    );
    let person_id = PersonID("2000".to_string());
    let runtime = Runtime::new().expect("unable to create runtime to test get person");
    // When
    let got = runtime.block_on(person_service.get_person(person_id));
    // Then
    match got {
        Ok(person) => panic!("unexpected result {:?}", person),
        Err(err) => assert_eq!(err, error::Error::GetPersonError),
    }
}

#[test]
fn test_create_person() {
    // Given
    let person = Person {
        id: PersonID("3".to_string()),
        name: "esme".to_string(),
    };
    let want = Person {
        id: PersonID("3".to_string()),
        name: "esme".to_string(),
    };
    let new_person = NewPerson::new("esme".to_string());
    let a_censor = DummyCensor::new("esme".to_string(), false);
    let a_store = DummyStore::new_with_add_person(Some(person), false);
    let person_service = service::Service::new(a_store, a_censor);
    let runtime = Runtime::new().expect("unable to create runtime to test create person");
    // When
    let got = runtime.block_on(person_service.add_person(new_person));
    // Then
    assert_eq!(false, got.is_err());

    match got {
        Ok(person) => assert_eq!(want, person),
        Err(err) => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn test_delete_person() {
    // Given
    let a_store = DummyStore::new_with_delete_person(true, false);
    let a_censor = DummyCensor::new("".to_string(), false);
    let person_service = service::Service::new(a_store, a_censor);
    let person_id = PersonID("2".to_string());
    let want = true;
    let runtime = Runtime::new().expect("unable to create runtime to test delete person");
    // When
    let got = runtime.block_on(person_service.delete_person(person_id));
    // Then
    match got {
        Ok(result_got) => assert_eq!(want, result_got),
        Err(err) => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn test_delete_person_but_not_found() {
    // Given
    let a_store = DummyStore::new_with_delete_person(false, true);
    let a_censor = DummyCensor::new("".to_string(), false);
    let person_service = service::Service::new(a_store, a_censor);
    let person_id = PersonID("2000".to_string());
    let runtime = Runtime::new().expect("unable to create runtime to test delete person");
    // When
    let got = runtime.block_on(person_service.delete_person(person_id));
    // Then
    match got {
        Ok(result) => panic!("unexpected result {}", result),
        Err(err) => assert_eq!(err, error::Error::DeletePersonError),
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
    let a_store = DummyStore::new_with_update_person(person_to_return, false);
    let a_censor = DummyCensor::new("".to_string(), false);
    let person_service = service::Service::new(a_store, a_censor);
    let want = Person {
        id: PersonID("1".to_string()),
        name: "Luisfer".to_string(),
    };
    let runtime = Runtime::new().expect("unable to create runtime to test update person");
    // When
    let got = runtime.block_on(person_service.update_person(a_person));
    // Then
    match got {
        Ok(got_person) => assert_eq!(want, got_person),
        Err(err) => panic!("unexpected value: {:?}", err),
    }
}

#[test]
fn test_update_person_but_not_found() {
    // Given
    let a_person = Person {
        id: PersonID("1".to_string()),
        name: "Luisfer".to_string(),
    };
    let a_store = DummyStore::new_with_update_person(None, true);
    let a_censor = DummyCensor::new("".to_string(), false);
    let person_service = service::Service::new(a_store, a_censor);
    let runtime = Runtime::new().expect("unable to create runtime to test update person");
    // When
    let got = runtime.block_on(person_service.update_person(a_person));
    // Then
    match got {
        Ok(person) => panic!("unexpected result {:?}", person),
        Err(err) => assert_eq!(err, error::Error::UpdatePersonError),
    }
}

#[derive(Debug, Clone)]
struct DummyCensor {
    response: String,
    is_error: bool,
}

impl DummyCensor {
    fn new(response: String, is_error: bool) -> Self {
        DummyCensor {
            response: response,
            is_error: is_error,
        }
    }
}

#[async_trait]
impl censor::Censorious for DummyCensor {
    async fn censor(&self, word: String) -> Result<String, error::Error> {
        match self.is_error {
            true => Err(error::Error::ValidateBadWordsError),
            false => {
                if self.response.is_empty() {
                    return Ok(word.clone());
                }
                return Ok(self.response.clone());
            }
        }
    }
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
    fn new_with_delete_person(result: bool, is_error: bool) -> Self {
        let mut dummy_store = DummyStore::default();
        dummy_store.delete_person_error = Some(is_error);
        dummy_store.delete_person_value = Some(result);

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
            true => Err(error::Error::GetPeopleError),
        }
    }

    async fn get_person(&self, _: PersonID) -> Result<Person, error::Error> {
        match &self.get_person_error.unwrap() {
            false => Ok(self.get_person_value.clone().unwrap()),
            true => Err(error::Error::GetPersonError),
        }
    }

    async fn add_person(&self, _: Person) -> Result<Person, error::Error> {
        match &self.add_person_error.unwrap() {
            false => Ok(self.add_person_value.clone().unwrap()),
            true => Err(error::Error::CreatePersonError),
        }
    }

    async fn update_person(&self, _: Person) -> Result<Person, error::Error> {
        match &self.update_person_error.unwrap() {
            false => Ok(self.update_person_value.clone().unwrap()),
            true => Err(error::Error::UpdatePersonError),
        }
    }

    async fn delete_person(&self, _: PersonID) -> Result<bool, error::Error> {
        match &self.delete_person_error.unwrap() {
            false => Ok(self.delete_person_value.unwrap()),
            true => Err(error::Error::DeletePersonError),
        }
    }

    async fn add_pet(&self, _: Pet) -> Result<Pet, error::Error> {
        match &self.add_pet_error.unwrap() {
            false => Ok(self.add_pet_value.clone().unwrap()),
            true => Err(error::Error::AddPetError),
        }
    }
}
