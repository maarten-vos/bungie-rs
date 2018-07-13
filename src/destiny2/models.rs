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
pub struct DestinyItemResponse {
    #[serde(rename = "characterId")]
    pub character_id: Option<i64>,
    pub item: Option<SingleComponentResponseOfDestinyItemComponent>,
    pub instance: Option<SingleComponentResponseOfDestinyItemInstanceComponent>,
    pub objectives: Option<SingleComponentResponseOfDestinyItemObjectivesComponent>,
    pub perks: Option<SingleComponentResponseOfDestinyItemPerksComponent>,
    #[serde(rename = "renderData")]
    pub render_data: Option<SingleComponentResponseOfDestinyItemRenderComponent>,
    pub stats: Option<SingleComponentResponseOfDestinyItemStatsComponent>,
    #[serde(rename = "talentGrid")]
    pub talent_grid: Option<SingleComponentResponseOfDestinyItemTalentGridComponent>,
    pub sockets: Option<SingleComponentResponseOfDestinyItemSocketsComponent>
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemSocketsComponent {
    pub data: DestinyItemSocketsComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemSocketsComponent {
    pub sockets: Vec<DestinyItemSocketState>
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemSocketState {
    #[serde(rename = "plugHash")]
    pub plug_hash: Option<u32>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "enableFailIndexes")]
    pub enable_fail_indexes: Vec<i32>,
    #[serde(rename = "reusablePlugHashes")]
    pub reusable_plug_hashes: Vec<u32>,
    #[serde(rename = "plugObjectives")]
    pub plug_objectives: Vec<DestinyObjectiveProgress>,
    #[serde(rename = "reusablePlugs")]
    pub reusable_plugs: Vec<DestinyItemPlug>
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemPlug {
    #[serde(rename = "plugItemHash")]
    pub plug_item_hash: u32,
    #[serde(rename = "plugObjectives")]
    pub plug_objectives: Vec<DestinyObjectiveProgress>,
    #[serde(rename = "canInsert")]
    pub can_insert: bool,
    pub enabled: bool,
    #[serde(rename = "insertFailIndexes")]
    pub insert_fail_indexes: Vec<i32>,
    #[serde(rename = "enableFailIndexes")]
    pub enable_fail_indexes: Vec<i32>
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemTalentGridComponent {
    pub data: DestinyItemTalentGridComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemTalentGridComponent {
    #[serde(rename = "talentGridHash")]
    pub talent_grid_hash: u32,
    pub nodes: Vec<DestinyTalentNode>,
    #[serde(rename = "isGridComplete")]
    pub is_grid_complete: bool,
    #[serde(rename = "gridProgression")]
    pub grid_progression: DestinyProgression
}

#[derive(Debug, Deserialize)]
pub struct DestinyTalentNode {
    #[serde(rename = "nodeIndex")]
    pub node_index: i32,
    #[serde(rename = "nodeHash")]
    pub node_hash: u32,
    pub state: DestinyTalentNodeState,
    #[serde(rename = "isActivated")]
    pub is_activated: bool,
    #[serde(rename = "stepIndex")]
    pub step_index: i32,
    #[serde(rename = "materialsToUpgrade")]
    pub materials_to_upgrade: Vec<DestinyMaterialRequirement>,
    #[serde(rename = "activationGridLevel")]
    pub activation_grid_level: i32,
    #[serde(rename = "progressPercent")]
    pub progress_percent: f64,
    pub hidden: bool,
    #[serde(rename = "nodeStatsBlock")]
    pub node_stats_block: DestinyTalentNodeStatBlock
}

#[derive(Debug, Deserialize)]
pub struct DestinyTalentNodeStatBlock {
    #[serde(rename = "currentStepStats")]
    pub current_step_stats: Vec<DestinyStat>,
    #[serde(rename = "nextStepStats")]
    pub next_step_stats: Vec<DestinyStat>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMaterialRequirement {
    #[serde(rename = "itemHash")]
    pub item_hash: u32,
    #[serde(rename = "deleteOnAction")]
    pub delete_on_action: bool,
    pub count: i32,
    #[serde(rename = "omitFromRequirements")]
    pub omit_from_requirements: bool
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemStatsComponent {
    pub data: DestinyItemStatsComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemStatsComponent {
    pub stats: HashMap<u32, DestinyStatDefinition>
}

#[derive(Debug, Deserialize)]
pub struct DestinyStatDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "aggregationType")]
    pub aggregation_type: DestinyStatAggregationType,
    #[serde(rename = "hasComputedBlock")]
    pub has_computed_block: bool,
    #[serde(rename = "statCategory")]
    pub stat_category: DestinyStatCategory,
    pub hash: u32,
    pub index: i32,
    pub redacted: bool
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemRenderComponent {
    pub data: DestinyItemRenderComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemRenderComponent {
    #[serde(rename = "useCustomDyes")]
    pub use_custom_dyes: bool,
    #[serde(rename = "artRegions")]
    pub art_regions: HashMap<i32, i32>
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemPerksComponent {
    pub data: DestinyItemPerksComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemPerksComponent {
    pub perks: Vec<DestinyPerkReference>
}

#[derive(Debug, Deserialize)]
pub struct DestinyPerkReference {
    #[serde(rename = "perkHash")]
    pub perk_hash: u32,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub visible: bool
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemObjectivesComponent {
    pub data: DestinyItemObjectivesComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemObjectivesComponent {
    pub objectives: Vec<DestinyObjectiveProgress>,
    #[serde(rename = "flavorObjective")]
    pub flavor_objective: DestinyObjectiveProgress,
    #[serde(rename = "dateCompleted")]
    pub date_completed: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemInstanceComponent {
    pub data: DestinyItemInstanceComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemInstanceComponent {
    #[serde(rename = "damageType")]
    pub damage_type: DamageType,
    #[serde(rename = "damageTypeHash")]
    pub damage_type_hash: Option<u32>,
    #[serde(rename = "primaryStat")]
    pub primary_stat: DestinyStat,
    #[serde(rename = "itemLevel")]
    pub item_level: i32,
    pub quality: i32,
    #[serde(rename = "isEquipped")]
    pub is_equipped: bool,
    #[serde(rename = "canEquip")]
    pub can_equip: bool,
    #[serde(rename = "equipRequiredLevel")]
    pub equip_required_level: i32,
    #[serde(rename = "unlockHashesRequiredToEquip")]
    pub unlock_hashes_required_to_equip: Vec<u32>,
    #[serde(rename = "cannotEquipReason")]
    pub cannot_equip_reason: EquipFailureReason
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyItemComponent {
    pub data: DestinyItemComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyStat {
    #[serde(rename = "statHash")]
    pub stat_hash: u32,
    pub value: i32,
    #[serde(rename = "maximumValue")]
    pub maximum_value: i32
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
    pub supplemental_display_name: Option<String>,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "membershipType")]
    pub membership_type: MembershipType,
    #[serde(rename = "membershipId")]
    pub membership_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinyItemActionRequest {
    #[serde(rename = "itemId")]
    pub item_id: i64,
    #[serde(rename = "characterId")]
    pub character_id: i64,
    #[serde(rename = "membershipType")]
    pub membership_type: MembershipType
}

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterResponse {
    pub inventory: Option<SingleComponentResponseOfDestinyInventoryComponent>,
    pub character: Option<SingleComponentResponseOfDestinyCharacterComponent>,
    pub progressions: Option<SingleComponentResponseOfDestinyCharacterProgressionComponent>,
    #[serde(rename = "renderData")]
    pub render_data: Option<SingleComponentResponseOfDestinyCharacterRenderComponent>
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterRenderComponent {
    pub data: DestinyCharacterRenderComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterRenderComponent {
    #[serde(rename = "customDyes")]
    pub custom_dyes: Vec<DyeReference>,
    pub customization: DestinyCharacterCustomization,
    #[serde(rename = "peerView")]
    pub peer_view: DestinyCharacterPeerView
}

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterPeerView {
    pub equipment: Vec<DestinyItemPeerView>
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemPeerView {
    #[serde(rename = "itemHash")]
    pub item_hash: u32,
    pub dyes: Vec<DyeReference>
}

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterCustomization {
    pub personality: u32,
    pub face: u32,
    #[serde(rename = "skinColor")]
    pub skin_color: u32,
    #[serde(rename = "lipColor")]
    pub lip_color: u32,
    #[serde(rename = "eyeColor")]
    pub eye_color: u32,
    #[serde(rename = "hairColors")]
    pub hair_colors: Vec<u32>,
    #[serde(rename = "featureColors")]
    pub feature_colors: Vec<u32>,
    #[serde(rename = "decalColor")]
    pub decal_color: u32,
    #[serde(rename = "wearHelmet")]
    pub wear_helmet: bool,
    #[serde(rename = "hairIndex")]
    pub hair_index: i32,
    #[serde(rename = "featureIndex")]
    pub feature_index: i32,
    #[serde(rename = "decalIndex")]
    pub decal_index: i32
}

#[derive(Debug, Deserialize)]
pub struct DyeReference {
    #[serde(rename = "channelHash")]
    pub channel_hash: u32,
    #[serde(rename = "dyeHash")]
    pub dye_hash: u32
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterProgressionComponent {
    pub data: DestinyCharacterProgressionComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterProgressionComponent {
    pub progressions: HashMap<u32, DestinyProgression>,
    pub factions: HashMap<u32, DestinyFactionProgression>,
    pub milestones: HashMap<u32, DestinyMilestone>,
    pub quests: Vec<DestinyQuestStatus>,
    #[serde(rename = "uninstancedItemObjectives")]
    pub uninstanced_item_objectives: HashMap<u32, Vec<DestinyInventoryItemDefinition>>
}

#[derive(Debug, Deserialize)]
pub struct DestinyInventoryItemDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: DestinyDisplayPropertiesDefinition,
    #[serde(rename = "secondaryIcon")]
    pub secondary_icon: String,
    #[serde(rename = "secondaryOverlay")]
    pub secondary_overlay: String,
    #[serde(rename = "secondarySpecial")]
    pub secondary_special: String,
    #[serde(rename = "backgroundColor")]
    pub background_color: DestinyColor,
    pub screenshot: String,
    #[serde(rename = "itemTypeDisplayName")]
    pub item_type_display_name: String,
    #[serde(rename = "uiItemDisplayStyle")]
    pub ui_item_display_style: String,
    #[serde(rename = "itemTypeAndTierDisplayName")]
    pub item_type_and_tier_display_name: String,
    #[serde(rename = "displaySource")]
    pub display_source: String,
    #[serde(rename = "tooltipStyle")]
    pub tooltip_style: String,
    pub action: DestinyItemActionBlockDefinition
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemActionBlockDefinition {
    #[serde(rename = "verbName")]
    pub verb_name: String
}

#[derive(Debug, Deserialize)]
pub struct DestinyDisplayPropertiesDefinition {
    pub description: String,
    pub name: String,
    pub icon: String,
    #[serde(rename = "hasIcon")]
    pub has_icon: bool
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestone {
    #[serde(rename = "milestoneHash")]
    pub milestone_hash: u32,
    #[serde(rename = "availableQuests")]
    pub available_quests: Vec<DestinyMilestoneQuest>,
    pub values: HashMap<String, f64>,
    #[serde(rename = "vendorHashes")]
    pub vendor_hashes: Vec<u32>,
    pub vendors: Vec<DestinyMilestoneVendor>,
    pub rewards: Vec<DestinyMilestoneRewardCategory>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneRewardCategory {
    #[serde(rename = "rewardCategoryHash")]
    pub reward_category_hash: u32,
    pub entries: Vec<DestinyMilestoneRewardEntry>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneRewardEntry {
    #[serde(rename = "rewardEntryHash")]
    pub reward_entry_hash: u32,
    pub earned: bool,
    pub redeemed: bool
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneVendor {
    #[serde(rename = "vendorHash")]
    pub vendor_hash: u32,
    #[serde(rename = "previewItemHash")]
    pub preview_item_hash: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneQuest {
    #[serde(rename = "questItemHash")]
    pub quest_item_hash: u32,
    pub status: DestinyQuestStatus,
    pub activity: Option<DestinyMilestoneActivity>,
    pub challenges: Vec<DestinyChallengeStatus>
}

#[derive(Debug, Deserialize)]
pub struct DestinyChallengeStatus {
    pub objective: DestinyObjectiveProgress
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneActivity {
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,
    #[serde(rename = "activityModeHash")]
    pub activity_mode_hash: Option<u32>,
    #[serde(rename = "activityModeType")]
    pub activity_mode_type: Option<ActivityModeType>,
    #[serde(rename = "modifierHashes")]
    pub modifier_hashes: Vec<u32>,
    pub variants: Vec<DestinyMilestoneActivityVariant>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneActivityVariant {
    #[serde(rename = "activityHash")]
    pub activity_hash: u32,
    #[serde(rename = "completionStatus")]
    pub completion_status: DestinyMilestoneActivityCompletionStatus,
    #[serde(rename = "activityModeHash")]
    pub activity_mode_hash: Option<u32>,
    #[serde(rename = "activityModeType")]
    pub activity_mode_type: Option<ActivityModeType>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneActivityCompletionStatus {
    pub completed: bool,
    pub phases: Vec<DestinyMilestoneActivityPhase>
}

#[derive(Debug, Deserialize)]
pub struct DestinyMilestoneActivityPhase {
    pub complete: bool
}

#[derive(Debug, Deserialize)]
pub struct DestinyQuestStatus {
    #[serde(rename = "questHash")]
    pub quest_hash: u32,
    #[serde(rename = "stepHash")]
    pub step_hash: u32,
    #[serde(rename = "stepObjectives")]
    pub step_objectives: Vec<DestinyObjectiveProgress>,
    pub tracked: bool,
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: i32,
    pub completed: bool,
    pub redeemed: bool,
    pub started: bool,
    #[serde(rename = "vendorHash")]
    pub vendor_hash: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct DestinyObjectiveProgress {
    #[serde(rename = "objectiveHash")]
    pub objective_hash: u32,
    #[serde(rename = "destinationHash")]
    pub destination_hash: Option<u32>,
    #[serde(rename = "activityHash")]
    pub activity_hash: Option<u32>,
    pub progress: Option<i32>,
    pub complete: bool,
    pub visible: bool
}

#[derive(Debug, Deserialize)]
pub struct DestinyFactionProgression {
    #[serde(rename = "factionHash")]
    pub faction_hash: u32,
    #[serde(rename = "factionVendorIndex")]
    pub faction_vendor_index: i32,
    #[serde(rename = "progressionHash")]
    pub progression_hash: u32,
    #[serde(rename = "dailyProgress")]
    pub daily_progress: i32,
    #[serde(rename = "dailyLimit")]
    pub daily_limit: i32,
    #[serde(rename = "weeklyProgress")]
    pub weekly_progress: i32,
    #[serde(rename = "weeklyLimit")]
    pub weekly_limit: i32,
    #[serde(rename = "currentProgress")]
    pub current_progress: i32,
    pub level: i32,
    #[serde(rename = "levelCap")]
    pub level_cap: i32,
    #[serde(rename = "stepIndex")]
    pub step_index: i32,
    #[serde(rename = "progressToNextLevel")]
    pub progress_to_next_level: i32,
    #[serde(rename = "nextLevelAt")]
    pub next_level_at: i32
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyCharacterComponent {
    pub data: DestinyCharacterComponent,
    pub privacy: ComponentPrivacySetting
} 

#[derive(Debug, Deserialize)]
pub struct DestinyCharacterComponent {
    #[serde(rename = "membershipId")]
    pub membership_id: i64,
    #[serde(rename = "membershipType")]
    pub membership_type: MembershipType,
    #[serde(rename = "characterId")]
    pub character_id: i64,
    #[serde(rename = "dateLastPlayed")]
    pub date_last_played: String,
    #[serde(rename = "minutesPlayedThisSession")]
    pub minutes_played_this_session: i64,
    #[serde(rename = "minutesPlayedTotal")]
    pub minutes_played_total: i64,
    pub light: i32,
    pub stats: HashMap<i32, u32>,
    #[serde(rename = "raceHash")]
    pub race_hash: u32,
    #[serde(rename = "genderHash")]
    pub gender_hash: u32,
    #[serde(rename = "classHash")]
    pub class_hash: u32,
    #[serde(rename = "raceType")]
    pub race_type: DestinyRace,
    #[serde(rename = "classType")]
    pub class_type: DestinyClass,
    #[serde(rename = "genderType")]
    pub gender_type: DestinyGender,
    #[serde(rename = "emblemPath")]
    pub emblem_path: String,
    #[serde(rename = "emblemColor")]
    pub emblem_color: DestinyColor,
    #[serde(rename = "levelProgression")]
    pub level_progression: DestinyProgression,
    #[serde(rename = "baseCharacterLevel")]
    pub base_character_level: i32,
    #[serde(rename = "percentToNextLevel")]
    pub percent_to_next_level: i32
}

#[derive(Debug, Deserialize)]
pub struct DestinyProgression {
    #[serde(rename = "progressionHash")]
    pub progression_hash: u32,
    #[serde(rename = "dailyProgress")]
    pub daily_progress: i32,
    #[serde(rename = "dailyLimit")]
    pub daily_limit: i32,
    #[serde(rename = "weeklyProgress")]
    pub weekly_progress: i32,
    #[serde(rename = "weeklyLimit")]
    pub weekly_limit: i32,
    #[serde(rename = "currentProgress")]
    pub current_progress: i32,
    pub level: i32,
    #[serde(rename = "levelCap")]
    pub level_cap: i32,
    #[serde(rename = "stepIndex")]
    pub step_index: i32,
    #[serde(rename = "progressToNextLevel")]
    pub progress_to_next_level: i32,
    #[serde(rename = "nextLevelAt")]
    pub next_level_at: i32
}

#[derive(Debug, Deserialize)]
pub struct DestinyColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

#[derive(Debug, Deserialize)]
pub struct SingleComponentResponseOfDestinyInventoryComponent {
    pub data: DestinyInventoryComponent,
    pub privacy: ComponentPrivacySetting
}

#[derive(Debug, Deserialize)]
pub struct DestinyInventoryComponent {
    pub items: Vec<DestinyItemComponent>
}

#[derive(Debug, Deserialize)]
pub struct DestinyItemComponent {
    #[serde(rename = "itemHash")]
    pub item_hash: u32,
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: i64,
    pub quantity: i32,
    #[serde(rename = "bindStatus")]
    pub bind_status: ItemBindStatus,
    pub location: ItemLocation,
    #[serde(rename = "bucketHash")]
    pub bucket_hash: u32,
    #[serde(rename = "transferStatus")]
    pub transfer_status: TransferStatuses,
    pub lockable: bool,
    pub state: ItemState
}

enum_number!(DestinyTalentNodeState {
    Invalid = 0,
    CanUpgrade = 1,
    NoPoints = 2,
    NoPrerequisites = 3,
    NoSteps = 4,
    NoUnlock = 5,
    NoMaterial = 6,
    NoGridLevel = 7,
    SwappingLocked = 8,
    MustSwap = 9,
    Complete = 10,
    Unknown = 11,
    CreationOnly = 12,
    Hidden = 13,
});

enum_number!(DestinyStatCategory {
    Gameplay = 0,
    Weapon = 1,
    Defense = 2,
    Primary = 3,
});

enum_number!(DestinyStatAggregationType {
    CharacterAverage = 0,
    Character = 1,
    Item = 2,
});

enum_number!(EquipFailureReason {
    None = 0,
    ItemUnequippable = 1,
    ItemUniqueEquipRestricted = 2,
    ItemFailedUnlockCheck = 4,
    ItemFailedLevelCheck = 8,
    ItemNotOnCharacter = 16,
});

enum_number!(DamageType {
    None = 0,
    Kinetic = 1,
    Arc = 2,
    Thermal = 3,
    Void = 4,
    Raid = 5,
});

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