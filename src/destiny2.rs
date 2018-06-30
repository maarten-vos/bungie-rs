use BungieClient;
use models::{Response, DestinyManifest};

pub struct Destiny2<'a> {
    pub bungie: &'a BungieClient
}

impl<'a> Destiny2<'a> {
    pub fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, ::failure::Error> {
        self.bungie.send_request("/Destiny2/Manifest/", None)
    }
}