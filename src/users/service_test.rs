use crate::errors::error;
use crate::types::accounts::{Account, AccountID, Login, NewAccount};
use crate::users::{service, storage};
use async_trait::async_trait;
use tokio::runtime::Runtime;

#[test]
fn test_create_account() {
    // Given
    let a_email = "aname@adomain.com".to_string();
    let a_password = "sfsfsf".to_string();
    let new_account = NewAccount::new(a_email, a_password);
    let a_store = DummyStore::new_with_add_account(false);
    let account_service = service::Service::new(a_store);
    let runtime = Runtime::new().expect("unable to create runtime to test create account");
    // When
    let got = runtime.block_on(account_service.add_account(new_account));
    // Then
    assert_eq!(false, got.is_err());
    let id = got.unwrap();
    assert_eq!(false, id.to_string().is_empty());
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
    let got = runtime.block_on(account_service.login(login));
    // Then
    assert_eq!(false, got.is_err());
    let result = got.unwrap();
    assert_eq!(true, !result.is_empty());
}

#[derive(Debug, Clone)]
struct DummyStore {
    login_error: Option<bool>,
    get_account_value: Option<Account>,
    add_account_error: Option<bool>,
}

impl DummyStore {
    fn new_with_login(is_error: bool, account: Account) -> Self {
        let mut dummy_store = DummyStore::default();
        dummy_store.login_error = Some(is_error);
        dummy_store.get_account_value = Some(account);
        dummy_store
    }
    fn new_with_add_account(is_error: bool) -> Self {
        let mut dummy_store = DummyStore::default();
        dummy_store.add_account_error = Some(is_error);
        dummy_store
    }
}

impl Default for DummyStore {
    fn default() -> DummyStore {
        DummyStore {
            login_error: Default::default(),
            get_account_value: Default::default(),
            add_account_error: Default::default(),
        }
    }
}

#[async_trait]
impl storage::Storer for DummyStore {
    async fn add_account(&self, account: Account) -> Result<AccountID, error::Error> {
        match &self.add_account_error.unwrap() {
            false => Ok(account.id.clone()),
            true => Err(error::Error::CreateAccountError),
        }
    }

    async fn get_account(&self, email: String) -> Result<Account, error::Error> {
        match &self.login_error.unwrap() {
            false => Ok(self.get_account_value.clone().unwrap()),
            true => Err(error::Error::GetAccountError),
        }
    }
}
