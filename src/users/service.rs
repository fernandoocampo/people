use crate::errors::error::Error;
use crate::{
    types::accounts::{AccountID, NewAccount},
    users::storage,
};
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub struct Service<T: storage::Storer> {
    store: T,
}

impl<T: storage::Storer> Service<T> {
    pub fn new(a_store: T) -> Self {
        Service { store: a_store }
    }

    pub async fn add_account(&self, new_account: NewAccount) -> Result<AccountID, Error> {
        debug!("start adding an account {:?}", new_account);

        let account = new_account.to_account();

        match self.store.add_account(account.clone()).await {
            Ok(id) => {
                info!("account with id: {} was created", id);
                Ok(id)
            }
            Err(e) => {
                if e == Error::DatabaseUniqueError {
                    error!("adding account {} : {:?}", account.email, e);
                    return Err(Error::DuplicateAccountError);
                }
                error!("adding account {} into repository: {:?}", account.email, e);
                Err(Error::CreateAccountError)
            }
        }
    }
}
