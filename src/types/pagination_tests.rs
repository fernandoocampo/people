#[cfg(test)]
mod pagination_tests {
    use crate::errors::error;
    use crate::types::pagination;
    use std::collections::HashMap;

    #[test]
    fn test_extract_pagination() {
        // Given
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("start"), String::from("1"));
        params.insert(String::from("end"), String::from("10"));

        let expected_result: Result<pagination::Pagination, error::Error> =
            Ok(pagination::Pagination { start: 1, end: 10 });
        // When
        let got = pagination::extract_pagination(params);
        // Then
        assert_eq!(got, expected_result);
    }

    #[test]
    fn test_extract_pagination_no_params() {
        // Given
        let params: HashMap<String, String> = HashMap::new();
        let expected_result = Err(error::Error::MissingParameters);
        // When
        let got = pagination::extract_pagination(params);
        // Then
        assert_eq!(got, expected_result);
    }

    #[test]
    fn test_extract_pagination_invalid_params() {
        // Given
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("start"), String::from("a"));
        params.insert(String::from("end"), String::from("10"));

        let expected_result: Result<pagination::Pagination, error::Error> = parse_int_error();
        // When
        let got = pagination::extract_pagination(params);
        // Then
        assert_eq!(got, expected_result);
    }

    fn parse_int_error() -> Result<pagination::Pagination, error::Error> {
        Ok(pagination::Pagination {
            start: "a".parse::<usize>().map_err(error::Error::ParseError)?,
            end: "1".parse::<usize>().map_err(error::Error::ParseError)?,
        })
    }
}
