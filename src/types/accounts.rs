use argon2::Config;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Account {
    pub fn verify_password(&self, password: &[u8]) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(&self.password, password)
    }

    pub fn issue_token(&self) -> String {
        let state = serde_json::to_string(&self.id).expect("failed to serialize state");
        local_paseto(state, "RANDOM WORDS WINTER MACINTOSH PC".as_bytes())
    }
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
            password: hash(self.password.as_bytes()),
        }
    }
}

impl fmt::Display for AccountID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Login {
    pub fn new(a_email: String, a_password: String) -> Self {
        Login {
            email: a_email,
            password: a_password,
        }
    }
}

impl fmt::Display for Login {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}

impl Default for Account {
    fn default() -> Account {
        Account {
            id: AccountID(String::from("")),
            email: String::from(""),
            password: String::from(""),
        }
    }
}

fn hash(password: &[u8]) -> String {
    let salt = rand::random::<[u8; 32]>();
    // you can use Config::default(), but that one will take too long.
    let config = Config::original();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn local_paseto(state: String, encryption_key: &[u8]) -> String {
    let dt = Utc::now() + Duration::hours(2);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(encryption_key))
        .set_issued_at(None)
        .set_expiration(&dt)
        .set_issuer("instructure")
        .set_audience("wizards")
        .set_jti("gandalf0")
        .set_not_before(&Utc::now())
        .set_subject("gandalf")
        .set_claim("go-to", json!(state))
        .set_footer("key-id:gandalf0")
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

#[cfg(test)]
mod account_test {
    use crate::types::accounts;

    #[test]
    fn test_hash_password() {
        // Given
        let a_password = "abcdefhi";
        // When
        let got = accounts::hash(a_password.as_bytes());
        // Then
        assert_eq!(false, got == a_password);
    }
}
