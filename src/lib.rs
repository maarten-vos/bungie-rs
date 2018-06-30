extern crate reqwest;
extern crate serde_json;

use destiny2::Destiny2;

mod models;
mod destiny2;

pub struct BungieClient {
    api_key: String,
    oauth_token: Option<String>
}

impl BungieClient {
    
    pub fn new(api_key: String) -> BungieClient {
        BungieClient { api_key, oauth_token: None }
    }

    pub fn with_authentication_token(&mut self, oauth_token: String) {
        self.oauth_token = Some(oauth_token);
    }

    pub fn destiny2(&self) -> Destiny2 {
        Destiny2 { bungie: &self }
    }

}