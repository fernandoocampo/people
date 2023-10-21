use crate::errors::error;
use crate::types::accounts::{Account, AccountID, NewAccount};
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

#[derive(Debug, Clone)]
struct DummyStore {
    add_account_error: Option<bool>,
}

impl DummyStore {
    fn new_with_add_account(is_error: bool) -> Self {
        let mut dummy_store = DummyStore::default();
        dummy_store.add_account_error = Some(is_error);
        dummy_store
    }
}

impl Default for DummyStore {
    fn default() -> DummyStore {
        DummyStore {
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
}
