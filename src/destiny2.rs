use lib::BungieClient;
use models::DestinyManifest;

struct Destiny2<'a> {
    bungie: &'a BungieClient
}

impl Destiny2 {
    fn get_destiny_manifest() -> DestinyManifest {
        bungie.send_request("/Destiny2/Manifest/");
    }
}