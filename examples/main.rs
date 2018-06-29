extern crate bungie;

use bungie::BungieClient;

fn main() {
    let bungie = BungieClient::new("<api-key>").with_authentication_token("<oauth-token-here>");
    let profile = bungie.destiny2().get_profile("membershipType", "membershipId");
}