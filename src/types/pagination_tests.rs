#[cfg(test)]
mod pagination_tests {
    use crate::errors::error;
    use crate::types::pagination;
    use std::collections::HashMap;

    #[test]
    fn test_extract_pagination() {
        // Given
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("offset"), String::from("0"));
        params.insert(String::from("limit"), String::from("10"));

        let expected_result: Result<pagination::Pagination, error::Error> =
            Ok(pagination::Pagination {
                offset: 0,
                limit: Some(10),
            });
        // When
        let got = pagination::extract_pagination(params);
        // Then
        assert_eq!(got.unwrap(), expected_result.unwrap());
    }

    #[test]
    fn test_extract_pagination_no_params() {
        // Given
        let params: HashMap<String, String> = HashMap::new();
        let expected_result = error::Error::MissingParameters;
        // When
        let got = pagination::extract_pagination(params);
        // Then
        assert_eq!(got.is_err(), true);
        match got {
            Ok(v) => panic!("unexpected result {:?}", v),
            Err(err) => assert_eq!(err, expected_result),
        }
    }

    #[test]
    fn test_extract_pagination_invalid_params() {
        // Given
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("offset"), String::from("a"));
        params.insert(String::from("limit"), String::from("10"));

        let expected_result: Result<pagination::Pagination, error::Error> = parse_int_error();
        // When
        let got = pagination::extract_pagination(params);
        // Then
        assert_eq!(got.is_err(), true);
        assert_eq!(got, expected_result);
    }

    fn parse_int_error() -> Result<pagination::Pagination, error::Error> {
        Ok(pagination::Pagination {
            offset: "a".parse::<i32>().map_err(error::Error::ParseError)?,
            limit: Some("10".parse::<i32>().map_err(error::Error::ParseError)?),
        })
    }
}
