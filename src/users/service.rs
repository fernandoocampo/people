use crate::errors::error::Error;
use crate::{
    types::accounts::{AccountID, Login, NewAccount},
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

    pub async fn login(&self, login: Login) -> Result<String, Error> {
        match self.store.get_account(login.email.clone()).await {
            Ok(account) => match account.verify_password(login.password.as_bytes()) {
                Ok(verified) => {
                    if verified {
                        Ok(account.issue_token())
                    } else {
                        Err(Error::WrongPasswordError)
                    }
                }
                Err(e) => {
                    error!("verifying login password for {} got: {:?}", login.email, e);
                    Err(Error::LoginError)
                }
            },
            Err(_) => Err(Error::GetAccountError),
        }
    }
}
