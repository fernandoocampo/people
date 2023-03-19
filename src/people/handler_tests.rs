#[cfg(test)]
mod handler_tests {
    use std::collections::HashMap;
    use crate::storage::memory::Store;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_people() {
        // Given
        let store = Store::new();
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("NV", "Nevada");
        params.insert("NY", "New York");
        // When
        // Then
    }
}