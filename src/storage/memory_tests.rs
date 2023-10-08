#[cfg(test)]
mod memory_tests {
    use crate::storage::memory::Store;
    use crate::types::people::{Person, PersonID};
    use tokio::runtime::Runtime;

    #[test]
    fn test_load_people() {
        // Given
        let store = Store::new();
        let expected_result: Vec<Person> = vec![
            Person {
                id: PersonID("1".to_string()),
                first_name: "Luis".to_string(),
                last_name: "Luis".to_string(),
            },
            Person {
                id: PersonID("2".to_string()),
                first_name: "Fernando".to_string(),
                last_name: "Fernando".to_string(),
            },
        ];
        let runtime = Runtime::new().expect("Unable to create a runtime");
        // When
        let mut got = runtime.block_on(get_people(store));
        got.sort();
        // Then
        assert_eq!(got, expected_result);
    }

    async fn get_people(store: Store) -> Vec<Person> {
        let res: Vec<Person> = store.people.read().await.values().cloned().collect();
        return res;
    }
}
