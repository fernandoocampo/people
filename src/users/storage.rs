use crate::types::accounts::Account;
use crate::{errors::error::Error, types::accounts::AccountID};
use async_trait::async_trait;
use std::fmt::{Debug, Error as FmtError, Formatter};

#[async_trait]
pub trait Storer {
    async fn add_account(&self, new_account: Account) -> Result<AccountID, Error>;
}

impl Debug for dyn Storer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        f.debug_struct("Storer").finish()
    }
}
