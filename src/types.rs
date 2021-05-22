use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;

pub trait Create<'a> {
    type Data: Serialize;
    type Response: DeserializeOwned;

    fn url(&self, base_url: &Url) -> Result<Url, Box<dyn Error>>;
    fn data(&'a self) -> Self::Data;
}

pub trait Delete {
    fn url(&self, base_url: &Url) -> Result<Url, Box<dyn Error>>;
}

pub trait Update<'a> {
    type Data: Serialize;

    fn url(&self, base_url: &Url) -> Result<Url, Box<dyn Error>>;
    fn data(&'a self) -> Self::Data;
}

pub trait Query {
    type Response;

    fn url(&self, base_url: &Url) -> Result<Url, Box<dyn Error>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBase {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,

    pub firstname: String,
    pub lastname: String,

    pub created_on: DateTime<Utc>,
    pub last_login_on: DateTime<Utc>,

    pub mail: Option<String>,
    pub login: Option<String>,
    pub api_key: Option<String>,
}
