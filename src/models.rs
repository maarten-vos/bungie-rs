use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Response<T> {
    #[serde(rename = "Response")]
    pub response: T
}

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

#[derive(Deserialize)]
pub struct GearAssetDataBaseDefinition {
    pub version: i32,
    pub path: String
}