use BungieClient;
use models::{Response, DestinyManifest, MembershipType, UserInfoCard};

pub struct Destiny2<'a> {
    pub bungie: &'a BungieClient
}

impl<'a> Destiny2<'a> {

    pub fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, ::failure::Error> {
        self.bungie.send_request("/Destiny2/Manifest/", None)
    }

    pub fn search_destiny_player(&self, membership_type: MembershipType, name: String) -> Result<Response<Vec<UserInfoCard>>, ::failure::Error> {
        let path = &format!("/Destiny2/SearchDestinyPlayer/{}/{}/", membership_type as i16, name);
        self.bungie.send_request(path, None)
    }

}