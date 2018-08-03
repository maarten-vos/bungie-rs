use BungieClient;
use self::models::*;
use std::fmt::Write;

pub mod models;

fn push_components(
    out: &mut String,
    mut components: Vec<DestinyComponentType>,
    default_component: DestinyComponentType
) {
    if components.is_empty() {
        components.push(default_component);
    }
    for item in &components {
        write!(out, "{},", *item as i64).unwrap();
    }
    out.pop();
}

pub struct Destiny2<'a> {
    pub bungie: &'a BungieClient
}

impl<'a> Destiny2<'a> {
    pub fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, ::failure::Error> {
        self.bungie.send_request("/Destiny2/Manifest", None)
    }

    pub fn search_destiny_player(&self, membership_type: MembershipType, name: &str) -> Result<Response<Vec<UserInfoCard>>, ::failure::Error> {
        let path = &format!("/Destiny2/SearchDestinyPlayer/{}/{}", membership_type as i64, name);
        self.bungie.send_request(path, None)
    }

    pub fn equip_item(&self, destiny_item_action_request: DestinyItemActionRequest) -> Result<Response<i32>, ::failure::Error> {
        self.bungie.send_request("/Destiny2/Actions/Items/EquipItem", Some(::serde_json::to_string(&destiny_item_action_request)?))
    }

    pub fn get_character(&self, membership_type: MembershipType, destiny_membership_id: &str, character_id: i64, components: Vec<DestinyComponentType>) -> Result<Response<DestinyCharacterResponse>, ::failure::Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}/Character/{}?components=", membership_type as i64, destiny_membership_id, character_id);
        push_components(&mut path, components, DestinyComponentType::Profiles);
        self.bungie.send_request(&path, None)
    }

    pub fn get_item(&self, membership_type: MembershipType, destiny_membership_id: &str, item_instance_id: i64, components: Vec<DestinyComponentType>) -> Result<Response<DestinyItemResponse>, ::failure::Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}/Item/{}?components=", membership_type as i64, destiny_membership_id, item_instance_id);
        push_components(&mut path, components, DestinyComponentType::None);
        self.bungie.send_request(&path, None)
    }

    pub fn get_profile(&self, membership_type: MembershipType, destiny_membership_id: &str, components: Vec<DestinyComponentType>) -> Result<Response<DestinyProfileResponse>, ::failure::Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}?components=", membership_type as i64, destiny_membership_id);
        push_components(&mut path, components, DestinyComponentType::Profiles);
        self.bungie.send_request(&path, None)
    }
}
