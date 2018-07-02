use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct GearAssetDataBaseDefinition {
    pub version: i32,
    pub path: String
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyItemActionRequest {
    #[serde(rename = "itemId")]
    item_id: i64,
    #[serde(rename = "characterId")]
    character_id: i64,
    #[serde(rename = "membershipType")]
    membership_type: MembershipType
}

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterResponse {
    inventory: Option<SingleComponentResponseOfDestinyInventoryComponent>,
    character: Option<SingleComponentResponseOfDestinyCharacterComponent>,
    progressions: Option<SingleComponentResponseOfDestinyCharacterProgressionComponent>
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterProgressionComponent {
    data: DestinyCharacterProgressionComponent,
    privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterProgressionComponent {
    progressions: HashMap<u32, DestinyProgression>,
    factions: HashMap<u32, DestinyFactionProgression>,
    milestones: HashMap<u32, DestinyMilestone>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestone {
    #[serde(rename = "milestoneHash")]
    milestone_hash: u32,
    #[serde(rename = "availableQuests")]
    available_quests: Vec<DestinyMilestoneQuest>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneQuest {
    #[serde(rename = "questItemHash")]
    quest_item_hash: u32,
    status: DestinyQuestStatus,
    activity: Option<DestinyMilestoneActivity>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneActivity {
    #[serde(rename = "activityHash")]
    activity_hash: u32,
    #[serde(rename = "activityModeHash")]
    activity_mode_hash: Option<u32>,
    #[serde(rename = "activityModeType")]
    activity_mode_type: Option<ActivityModeType>
}

#[derive(Debug, Deserialize)]
pub struct DestinyQuestStatus {
    #[serde(rename = "questHash")]
    quest_hash: u32,
    #[serde(rename = "stepHash")]
    step_hash: u32,
    #[serde(rename = "stepObjectives")]
    step_objectives: Vec<DestinyObjectiveProgress>,
    tracked: bool,
    #[serde(rename = "itemInstanceId")]
    item_instance_id: i32,
    completed: bool,
    redeemed: bool,
    started: bool,
    #[serde(rename = "vendorHash")]
    vendor_hash: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct DestinyObjectiveProgress {
    #[serde(rename = "objectiveHash")]
    objective_hash: u32,
    #[serde(rename = "destinationHash")]
    destination_hash: Option<u32>,
    #[serde(rename = "activityHash")]
    activity_hash: Option<u32>,
    progress: Option<i32>,
    complete: bool,
    visible: bool
}

#[derive(Debug, Deserialize)]
pub struct DestinyFactionProgression {
    #[serde(rename = "factionHash")]
    faction_hash: u32,
    #[serde(rename = "factionVendorIndex")]
    faction_vendor_index: i32,
    #[serde(rename = "progressionHash")]
    progression_hash: u32,
    #[serde(rename = "dailyProgress")]
    daily_progress: i32,
    #[serde(rename = "dailyLimit")]
    daily_limit: i32,
    #[serde(rename = "weeklyProgress")]
    weekly_progress: i32,
    #[serde(rename = "weeklyLimit")]
    weekly_limit: i32,
    #[serde(rename = "currentProgress")]
    current_progress: i32,
    level: i32,
    #[serde(rename = "levelCap")]
    level_cap: i32,
    #[serde(rename = "stepIndex")]
    step_index: i32,
    #[serde(rename = "progressToNextLevel")]
    progress_to_next_level: i32,
    #[serde(rename = "nextLevelAt")]
    next_level_at: i32
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterComponent {
    data: DestinyCharacterComponent,
    privacy: ComponentPrivacySetting
} 

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterComponent {
    #[serde(rename = "membershipId")]
    membership_id: i64,
    #[serde(rename = "membershipType")]
    membership_type: MembershipType,
    #[serde(rename = "characterId")]
    character_id: i64,
    #[serde(rename = "dateLastPlayed")]
    date_last_played: String,
    #[serde(rename = "minutesPlayedThisSession")]
    minutes_played_this_session: i64,
    #[serde(rename = "minutesPlayedTotal")]
    minutes_played_total: i64,
    light: i32,
    stats: HashMap<i32, u32>,
    #[serde(rename = "raceHash")]
    race_hash: u32,
    #[serde(rename = "genderHash")]
    gender_hash: u32,
    #[serde(rename = "classHash")]
    class_hash: u32,
    #[serde(rename = "raceType")]
    race_type: DestinyRace,
    #[serde(rename = "classType")]
    class_type: DestinyClass,
    #[serde(rename = "genderType")]
    gender_type: DestinyGender,
    #[serde(rename = "emblemPath")]
    emblem_path: String,
    #[serde(rename = "emblemColor")]
    emblem_color: DestinyColor,
    #[serde(rename = "levelProgression")]
    level_progression: DestinyProgression,
    #[serde(rename = "baseCharacterLevel")]
    base_character_level: i32,
    #[serde(rename = "percentToNextLevel")]
    percent_to_next_level: i32
}

#[derive(Debug, Deserialize)]
pub struct DestinyProgression {
    #[serde(rename = "progressionHash")]
    progression_hash: u32,
    #[serde(rename = "dailyProgress")]
    daily_progress: i32,
    #[serde(rename = "dailyLimit")]
    daily_limit: i32,
    #[serde(rename = "weeklyProgress")]
    weekly_progress: i32,
    #[serde(rename = "weeklyLimit")]
    weekly_limit: i32,
    #[serde(rename = "currentProgress")]
    current_progress: i32,
    level: i32,
    #[serde(rename = "levelCap")]
    level_cap: i32,
    #[serde(rename = "stepIndex")]
    step_index: i32,
    #[serde(rename = "progressToNextLevel")]
    progress_to_next_level: i32,
    #[serde(rename = "nextLevelAt")]
    next_level_at: i32
}

#[derive(Debug, Deserialize)]
pub struct DestinyColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyInventoryComponent {
    data: DestinyInventoryComponent,
    privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyInventoryComponent {
    items: Vec<DestinyItemComponent>
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemComponent {
    #[serde(rename = "itemHash")]
    item_hash: u32,
    #[serde(rename = "itemInstanceId")]
    item_instance_id: i64,
    quantity: i32,
    #[serde(rename = "bindStatus")]
    bind_status: ItemBindStatus,
    location: ItemLocation,
    #[serde(rename = "bucketHash")]
    bucket_hash: u32,
    #[serde(rename = "transferStatus")]
    transfer_status: TransferStatuses,
    lockable: bool,
    state: ItemState
}

enum_number!(ActivityModeType {
    None = 0,
    Story = 2,
    Strike = 3,
    Raid = 4,
    AllPvP = 5,
    Patrol = 6,
    AllPvE = 7,
    Reserved9 = 9,
    Control = 10,
    Reserved11 = 11,
    Clash = 12,
    Reserved13 = 13,
    CrimsonDoubles = 15,
    Nightfall = 16,
    HeroicNightfall = 17,
    AllStrikes = 18,
    IronBanner = 19,
    Reserved20 = 20,
    Reserved21 = 21,
    Reserved22 = 22,
    Reserved24 = 24,
    AllMayhem = 25,
    Reserved26 = 26,
    Reserved27 = 27,
    Reserved28 = 28,
    Reserved29 = 29,
    Reserved30 = 30,
    Supremacy = 31,
    PrivateMatchesAll = 32,
    Survival = 37,
    Countdown = 38,
    TrailsOfTheNine = 39,
    Social = 40,
    TrailsCountdown = 41,
    TrailsSurvival = 42,
    IronBannerControl = 43,
    IronBannerClash = 44,
    IronBannerSupremacy = 45,
    ScoredNightfall = 46,
    ScoredHeroicNightfall = 47,
    Rumble = 48,
    AllDoubles = 49,
    Doubles = 50,
    PrivateMatchesClash = 51,
    PrivateMatchesControl = 52,
    PrivateMatchesSupremacy = 53,
    PrivateMatchesCountdown = 54,
    PrivateMatchesSurvival = 55,
    PrivateMatchesMayhem = 56,
    PrivateMatchesRumble = 57,
    HeroicAdventure = 58,
    Showdown = 59,
});

enum_number!(DestinyGender {
    Male = 0,
    Female = 1,
    Unknown = 2,
});

enum_number!(DestinyClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    Unknown = 3,
});

enum_number!(DestinyRace {
    Human = 0,
    Awoken = 1,
    Exo = 2,
    Unknown = 3,
});

enum_number!(ComponentPrivacySetting {
    None = 0,
    Public = 1,
    Private = 2,
});

enum_number!(ItemState {
    None = 0,
    Locked = 1,
    Tracked = 2,
    Master = 4,
});

enum_number!(TransferStatuses {
    CanTransfer = 0,
    ItemIsEquipped = 1,
    NotTransferrable = 2,
    NoRoomInDestination = 4,
});

enum_number!(ItemLocation {
    Unknown = 0,
    Inventory = 1,
    Vault = 2,
    Vender = 3,
    Postmaster = 4,
});

enum_number!(ItemBindStatus {
    NotBound = 0,
    BoundToCharacter = 1,
    BoundToAccount = 2,
    BoundToGuild = 3,
});

enum_number!(MembershipType {
    None = 0,
    TigerXbox = 1,
    TigerPsn = 2,
    TigerBlizzard = 4,
    TigerDemon = 10,
    BungieNext = 254,
    All = -1,
});

enum_number!(DestinyComponentType {
    None = 0,
    Profiles = 100,
    VendorReceipts = 101,
    ProfileInventories = 102,
    ProfileCurrencies = 103,
    ProfileProgression = 104,
    Characters = 200,
    CharacterInventories = 201,
    CharacterProgressions = 202,
    CharacterRenderData = 203,
    CharacterActivities = 204,
    CharacterEquipment = 205,
    ItemInstances = 300,
    ItemObjectives = 301,
    ItemPerks = 302,
    ItemRenderData = 303,
    ItemStats = 304,
    ItemSockets = 305,
    ItemTalentGrids = 306,
    ItemCommonData = 307,
    ItemPlugStates = 308,
    Vendors = 400,
    VendorCategories = 401,
    VendorSales = 402,
    Kiosks = 500,
    CurrencyLookups = 600,
});