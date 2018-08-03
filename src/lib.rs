extern crate reqwest;
extern crate serde;
extern crate failure;
extern crate serde_json;
#[macro_use] extern crate hyper;
#[macro_use] extern crate serde_derive;

use destiny2::Destiny2;
use reqwest::{
    Client,
    header::Authorization
};
use serde::de::DeserializeOwned;

#[macro_use]
mod macros;
pub mod destiny2;

header! { (XApiKey, "X-API-Key") => [String] }

pub struct BungieClient {
    api_key: String,
    oauth_token: Option<String>
}

impl BungieClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key, oauth_token: None }
    }

    pub fn with_authentication_token(mut self, oauth_token: String) -> Self {
        self.oauth_token = Some(oauth_token);
        self
    }

    pub fn destiny2(&self) -> Destiny2 {
        Destiny2 { bungie: &self }
    }

    fn send_request<T: DeserializeOwned>(&self, path: &str, body: Option<String>) -> Result<T, failure::Error> {
        let client = Client::new();
        let path = "https://www.bungie.net/Platform".to_owned() + path;
        let mut request = if let Some(body) = body {
            let mut request = client.post(&path);
            request.body(body);
            request
        } else {
            client.get(&path)
        };
        if let Some(ref oauth_token) = self.oauth_token {
            request.header(Authorization(oauth_token.clone()));
        }
        request.header(XApiKey(self.api_key.clone()));
        let mut response = request.send()?;
        Ok(response.json()?)
    }
}
