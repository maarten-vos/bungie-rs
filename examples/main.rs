extern crate bungie;

use bungie::BungieClient;

fn main() {
    let bungie = BungieClient::new("<api-key>").with_authentication_token("<oauth-token>");
    let manifest = bungie.destiny2().get_destiny_manifest();
}