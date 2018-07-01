use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
#[derive(Deserialize)]
pub struct Response<T> {
    #[serde(rename = "Response")]
    pub response: T,
    #[serde(rename = "ErrorCode")]
    pub error_code: i32,
    #[serde(rename = "ThrottleSeconds")]
    pub throttle_seconds: i32,
    #[serde(rename = "ErrorStatus")]
    pub error_status: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "MessageData")]
    pub message_data: HashMap<String, String>,
    #[serde(rename = "DetailedErrorTrace")]
    pub detailed_error_trace: Option<String>
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct DestinyManifest {
    pub version: String,
    #[serde(rename = "mobileAssetContentPath")]
    pub mobile_asset_content_path: String,
    #[serde(rename = "mobileGearAssetDataBases")]
    pub mobile_gear_asset_databases: Vec<GearAssetDataBaseDefinition>,
    #[serde(rename = "mobileWorldContentPaths")]
    pub mobile_world_content_paths: HashMap<String, String>,
    #[serde(rename = "mobileClanBannerDatabasePath")]
    pub mobile_clan_banner_database_path: String,
    #[serde(rename = "mobileGearCDN")]
    pub mobile_gear_cdn: HashMap<String, String>
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct GearAssetDataBaseDefinition {
    pub version: i32,
    pub path: String
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct UserInfoCard {
    #[serde(rename = "supplementalDisplayName")]
    supplemental_display_name: Option<String>,
    #[serde(rename = "iconPath")]
    icon_path: String,
    #[serde(rename = "membershipType")]
    membership_type: MembershipType,
    #[serde(rename = "membershipId")]
    membership_id: String,
    #[serde(rename = "displayName")]
    display_name: String
}

enum_number!(MembershipType {
    None = 0,
    TigerXbox = 1,
    TigerPsn = 2,
    TigerBlizzard = 4,
    TigerDemon = 10,
    BungieNext = 254,
    All = -1,
});