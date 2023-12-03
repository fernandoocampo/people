#[cfg(test)]
mod handler_tests {
    use crate::{
        errors::error::Error,
        types::accounts::{Account, AccountID, Login, NewAccount, SaveAccountSuccess},
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

    #[test]
    fn test_login() {
        // Given
        let login = Login::new(
            String::from("myname@mydomain.com"),
            String::from("any_password"),
        );
        let a_new_account = NewAccount {
            email: "myname@mydomain.com".to_string(),
            password: "any_password".to_string(),
        };
        let existing_account = a_new_account.to_account();
        let a_store = DummyStore::new_with_login(false, existing_account);
        let account_service = service::Service::new(a_store);
        let runtime = Runtime::new().expect("unable to create runtime to test login");
        // When
        let got = runtime.block_on(handler::login(login, account_service));
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

        assert_eq!(true, !got_result.is_empty());
    }

    #[derive(Debug, Clone)]
    struct DummyStore {
        add_account_id_value: Option<AccountID>,
        add_account_error: bool,
        login_error: Option<bool>,
        get_account_value: Option<Account>,
    }

    impl DummyStore {
        fn new_add_account(account_id: Option<AccountID>, is_error: bool) -> Self {
            let mut dummy_store = DummyStore::default();
            dummy_store.add_account_id_value = account_id;
            dummy_store.add_account_error = is_error;

            dummy_store
        }

        fn new_with_login(is_error: bool, account: Account) -> Self {
            let mut dummy_store = DummyStore::default();
            dummy_store.login_error = Some(is_error);
            dummy_store.get_account_value = Some(account);
            dummy_store
        }
    }

    impl Default for DummyStore {
        fn default() -> DummyStore {
            DummyStore {
                add_account_id_value: Default::default(),
                add_account_error: Default::default(),
                login_error: Default::default(),
                get_account_value: Default::default(),
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

        async fn get_account(&self, _: String) -> Result<Account, Error> {
            match &self.login_error.unwrap() {
                false => Ok(self.get_account_value.clone().unwrap()),
                true => Err(Error::GetAccountError),
            }
        }
    }
}
