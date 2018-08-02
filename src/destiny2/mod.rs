use BungieClient;
use destiny2::models::*;

pub mod models;

fn get_components(mut components: Vec<DestinyComponentType>, default_component: DestinyComponentType) -> String {
    if components.is_empty() {
        components.push(default_component);
    }
    let mut component_list = components.iter().fold(String::new(), |string, elem| string + &format!("{}", *elem as i64));
    component_list.pop();
    component_list
}

pub struct Destiny2<'a> {
    pub bungie: &'a BungieClient
}

impl<'a> Destiny2<'a> {

    pub fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, ::failure::Error> {
        self.bungie.send_request("/Destiny2/Manifest", None)
    }

    pub fn search_destiny_player(&self, membership_type: &MembershipType, name: &str) -> Result<Response<Vec<UserInfoCard>>, ::failure::Error> {
        let path = &format!("/Destiny2/SearchDestinyPlayer/{}/{}", *membership_type as i64, name);
        self.bungie.send_request(path, None)
    }

    pub fn equip_item(&self, destiny_item_action_request: DestinyItemActionRequest) -> Result<Response<i32>, ::failure::Error> {
        self.bungie.send_request("/Destiny2/Actions/Items/EquipItem", Some(::serde_json::to_string(&destiny_item_action_request)?))
    }

    pub fn get_character(&self, membership_type: &MembershipType, destiny_membership_id: &str, character_id: i64, components: Vec<DestinyComponentType>) -> Result<Response<DestinyCharacterResponse>, ::failure::Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}/Character/{}?components=", *membership_type as i64, destiny_membership_id, character_id);
        path.push_str(&get_components(components, DestinyComponentType::Profiles));
        self.bungie.send_request(&path, None)
    }

    pub fn get_item(&self, membership_type: &MembershipType, destiny_membership_id: &str, item_instance_id: i64, components: Vec<DestinyComponentType>) -> Result<Response<DestinyItemResponse>, ::failure::Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}/Item/{}?components=", *membership_type as i64, destiny_membership_id, item_instance_id);
        path.push_str(&get_components(components, DestinyComponentType::None));
        self.bungie.send_request(&path, None)
    }

    pub fn get_profile(&self, membership_type: &MembershipType, destiny_membership_id: &str, components: Vec<DestinyComponentType>) -> Result<Response<DestinyProfileResponse>, ::failure::Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}?components=", *membership_type as i64, destiny_membership_id);
        path.push_str(&get_components(components, DestinyComponentType::Profiles));
        self.bungie.send_request(&path, None)
    }

}