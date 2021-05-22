use crate::types::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub user: User,
}

pub struct CurrentUser;

impl Query for CurrentUser {
    type Response = UserResponse;

    fn url(&self, base_url: &Url) -> Result<Url, Box<dyn Error>> {
        Ok(base_url.join("users/current.json")?)
    }
}

pub struct UserById(pub u64);

impl Query for UserById {
    type Response = UserResponse;

    fn url(&self, base_url: &Url) -> Result<Url, Box<dyn Error>> {
        Ok(base_url.join(&format!("users/{}.json", self.0))?)
    }
}
