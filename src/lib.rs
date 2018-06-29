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

}

struct Destiny2<'a> {
    bungie: &'a BungieClient
}