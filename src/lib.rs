use crate::types::*;
use log::debug;
use reqwest::{blocking::Client, header::HeaderName, IntoUrl, Method, Url};
use serde::de::DeserializeOwned;
use std::error::Error;

pub mod types;
pub mod queries;

pub struct Redmine {
    base_url: Url,
    api_key: String,
    client: Client,
}

impl Redmine {
    pub fn new<U>(base_url: U, api_key: String) -> Result<Redmine, Box<dyn Error>>
    where
        U: IntoUrl,
    {
        Ok(Redmine {
            base_url: base_url.into_url()?,
            api_key,
            client: Client::new(),
        })
    }

    pub fn query<Q>(&self, query: Q) -> Result<Q::Response, Box<dyn Error>>
    where
        Q: Query,
        Q::Response: DeserializeOwned,
    {
        let url = query.url(&self.base_url)?;

        debug!("GET request on {}", url);
        Ok(self
            .request(Method::GET, url)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn create<'a, T>(&self, item: &'a T) -> Result<T::Response, Box<dyn Error>>
    where
        T: Create<'a>,
    {
        let url = item.url(&self.base_url)?;

        debug!("POST request on {}", url);
        Ok(self
            .request(Method::POST, url)
            .json(&item.data())
            .send()?
            .error_for_status()?
            .json()?)

        // TODO: Check for bad response (Unauthorized etc)
    }

    pub fn update<'a, T>(&self, item: &'a T) -> Result<(), Box<dyn Error>>
    where
        T: Update<'a>,
    {
        let url = item.url(&self.base_url)?;

        debug!("PUT request on {}", url);
        self.request(Method::PUT, url)
            .json(&item.data())
            .send()?
            .error_for_status()?;

        Ok(())
        // TODO: Check for bad response (Unauthorized etc)
    }

    pub fn delete<T>(&self, item: &T) -> Result<(), Box<dyn Error>>
    where
        T: Delete,
    {
        let url = item.url(&self.base_url)?;

        debug!("DELETE request on {}", url);
        self.request(Method::DELETE, url)
            .send()?
            .error_for_status()?;

        Ok(())
        // TODO: Check for bad response (Unauthorized etc)
    }

    fn request(&self, method: Method, url: Url) -> reqwest::blocking::RequestBuilder {
        self.client.request(method, url).header(
            HeaderName::from_static("x-redmine-api-key"),
            self.api_key.as_str(),
        )
    }
}
