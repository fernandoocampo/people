#[cfg(test)]
mod handler_tests {
    use crate::{
        errors::error::Error,
        types::accounts::{Account, AccountID, NewAccount, SaveAccountSuccess},
        users::{handler, service, storage::Storer},
    };
    use async_trait::async_trait;
    use hyper::StatusCode;
    use tokio::runtime::Runtime;
    use warp::reply::Reply;

    #[test]
    fn test_register() {
        // Given
        let new_account = NewAccount {
            email: "any@anydomain.com".to_string(),
            password: "12345678".to_string(),
        };
        let account_id = AccountID("386edb59-f2df-4284-ab24-1c32d78da6a9".to_string());
        let expected_result = SaveAccountSuccess {
            id: "386edb59-f2df-4284-ab24-1c32d78da6a9".to_string(),
        };
        let a_store = DummyStore::new_add_account(Some(account_id), false);
        let account_service = service::Service::new(a_store);
        let runtime = Runtime::new().expect("unable to create runtime to test register account");
        // When
        let got = runtime.block_on(handler::register(new_account, account_service));
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

        let got_account_id: SaveAccountSuccess = serde_json::from_str(got_result.as_str()).unwrap();
        assert_eq!(got_account_id, expected_result);
    }

    #[derive(Debug, Clone)]
    struct DummyStore {
        add_account_id_value: Option<AccountID>,
        add_account_error: bool,
    }

    impl DummyStore {
        fn new_add_account(account_id: Option<AccountID>, is_error: bool) -> Self {
            let mut dummy_store = DummyStore::default();
            dummy_store.add_account_id_value = account_id;
            dummy_store.add_account_error = is_error;

            dummy_store
        }
    }

    impl Default for DummyStore {
        fn default() -> DummyStore {
            DummyStore {
                add_account_id_value: Default::default(),
                add_account_error: Default::default(),
            }
        }
    }

    #[async_trait]
    impl Storer for DummyStore {
        async fn add_account(&self, _: Account) -> Result<AccountID, Error> {
            match &self.add_account_error {
                true => Err(Error::CreateAccountError),
                false => Ok(self.add_account_id_value.clone().unwrap()),
            }
        }
    }
}
