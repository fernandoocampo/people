use serde::{Deserialize, Serialize};
use std::{
    fmt,
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SaveAccountSuccess {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountID(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: AccountID,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
}

impl SaveAccountSuccess {
    pub fn new(account_id: AccountID) -> Self {
        SaveAccountSuccess {
            id: account_id.to_string(),
        }
    }
}

impl FromStr for AccountID {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(AccountID(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

impl NewAccount {
    pub fn new(a_email: String, a_password: String) -> Self {
        NewAccount {
            email: a_email,
            password: a_password,
        }
    }

    pub fn to_account(&self) -> Account {
        Account {
            id: AccountID(uuid::Uuid::new_v4().to_string()),
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}

impl fmt::Display for AccountID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}