extern crate reqwest;
extern crate serde_json;

use destiny2;

mod models;

struct BungieClient {
    api_key: String,
    oauth_token: Option<String>
}

impl BungieClient {
    fn new(api_key: String) -> BungieClient {
        BungieClient { api_key, oauth_token: None }
    }

    fn with_authentication_token(&mut self, oauth_token: String) {
        self.oauth_token = Some(oauth_token);
    }

    fn destiny2(&self) -> Destiny2 {
        Destiny2 { bungie: &self }
    }

    fn send_request<T>(method: HttpMethod, path: &str) -> T {
        match method {
            HttpMethod::Get => {
                let response = reqwest::get("https://www.bungie.net/Platform".to_owned() + path);
            },
            HttpMethod::Post => {
                
            }
        }
    }

}

enum HttpMethod {
    Get,
    Post
}