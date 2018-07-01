use BungieClient;
use models::*;

pub struct Destiny2<'a> {
    pub bungie: &'a BungieClient
}

impl<'a> Destiny2<'a> {

    pub fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, ::failure::Error> {
        self.bungie.send_request("/Destiny2/Manifest", None)
    }

    pub fn search_destiny_player(&self, membership_type: MembershipType, name: String) -> Result<Response<Vec<UserInfoCard>>, ::failure::Error> {
        let path = &format!("/Destiny2/SearchDestinyPlayer/{}/{}", membership_type as i64, name);
        self.bungie.send_request(path, None)
    }

    pub fn equip_item(&self, destiny_item_action_request: DestinyItemActionRequest) -> Result<Response<i32>, ::failure::Error> {
        self.bungie.send_request("/Destiny2/Actions/Items/EquipItem", Some(::serde_json::to_string(&destiny_item_action_request)?))
    }

    pub fn get_character(&self, membership_type: MembershipType, destiny_membership_id: String, character_id: i64, mut components: Vec<DestinyComponentType>) -> Result<DestinyCharacterResponse, ::failure::Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}/Character/{}?components=", membership_type as i64, destiny_membership_id, character_id);
        if components.is_empty() {
            components.push(DestinyComponentType::Profiles);
        }
        let mut component_list = components.iter().fold(String::new(), |string, elem| string + &format!("{}", *elem as i64));
        component_list.pop();
        path.push_str(&component_list);
        self.bungie.send_request(&path, None)
    }

}