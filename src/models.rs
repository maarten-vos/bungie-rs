use std::collections::HashMap;

struct DestinyManifest {
    version: String,
    mobile_asset_content_path: String,
    mobile_gear_asset_databases: Vec<GearAssetDataBaseDefinition>,
    mobile_world_content_paths: HashMap<String, String>,
    mobile_clan_banner_database_path: String,
    mobile_gear_cdn: HashMap<String, String>
}

struct GearAssetDataBaseDefinition {
    version: i32,
    path: String
}