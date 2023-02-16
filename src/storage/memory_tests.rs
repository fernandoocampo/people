#[cfg(test)]
mod memory_tests {
    use crate::storage::memory::Store;
    use crate::types::people::{PersonID, Person};
    use tokio::runtime::Runtime;
    
    #[test]
    fn test_load_people() {
        // Given
        let store = Store::new();
        let expected_result: Vec<Person> = vec![
            Person {
                id: PersonID("1".to_string()),
                name: "Luis".to_string(),
            },
            Person {
                id: PersonID("2".to_string()),
                name: "Fernando".to_string(),
            }
        ];
        let runtime = Runtime::new().expect("Unable to create a runtime");
        // When
        let got = runtime.block_on(get_people(store));
        // Then
        assert_eq!(got, expected_result);
    }

    async fn get_people(store: Store) -> Vec<Person> {
        let res: Vec<Person> = store.people.read().await.values().cloned().collect();
        return res;
    }
}
