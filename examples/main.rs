extern crate bungie;

use bungie::BungieClient;

fn main() {
    let bungie = BungieClient::new("<api-key>".to_owned()).with_authentication_token("<oauth-token>".to_owned());
    let manifest = bungie.destiny2().get_destiny_manifest();
}