use {
  super::{
    CsgMod,
    Mod,
    Validation
  },
  serde::{
    Deserialize,
    Deserializer,
    Serialize
  }
};

fn serde_mods<'de, D>(deserializer: D) -> Result<Vec<Mod>, D::Error>
where
  D: Deserializer<'de>
{
  let mods: Vec<CsgMod> = Vec::deserialize(deserializer)?;
  Ok(mods.into_iter().map(Mod::CsgFormat).collect())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerSavegame {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub settings: Option<Settings>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub map: Option<Map>,
  #[serde(skip_serializing_if = "Option::is_none", rename = "introductionHelp")]
  pub introduction_help: Option<IntroductionHelp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub statistics: Option<CsgStatistics>,
  #[serde(skip_serializing_if = "Option::is_none", rename = "mapsSplitShapeFileIds")]
  pub maps_split_shape_file_ids: Option<MapsSplitShapeFileIds>,
  #[serde(skip_serializing_if = "Option::is_none", rename = "slotSystem")]
  pub slot_system: Option<SlotSystem>,
  #[cfg(feature = "fs25")]
  #[serde(skip_serializing_if = "Option::is_none", rename = "foliageTypes")]
  pub foliage_types: Option<FoliageTypes>,
  #[serde(rename = "mod", deserialize_with = "serde_mods", default)]
  pub mods: Vec<Mod>
}

impl Validation for CareerSavegame {
  fn is_valid(&self) -> bool {
    self
      .settings
      .as_ref()
      .map(|s| !s.map_title.is_empty() && s.time_scale > 0.0)
      .unwrap_or(false)
      && self.slot_system.as_ref().is_some_and(|ss| !ss.slot_usage.is_empty())
  }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
  pub savegame_name: Box<str>,
  pub creation_date: Box<str>,
  pub map_id: Box<str>,
  pub map_title: Box<str>,
  pub save_date_formatted: Box<str>,
  pub save_date: Box<str>,
  #[cfg(feature = "fs22")]
  #[serde(default)]
  pub reset_vehicles: bool,
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub initial_money: i32,
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub initial_loan: i32,
  #[cfg(feature = "fs22")]
  #[serde(default)]
  pub difficulty: i8,
  pub economic_difficulty: Box<str>,
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub has_initially_owned_farmlands: bool,
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub load_default_farm: bool,
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub start_with_guided_tour: bool,
  #[serde(default)]
  pub traffic_enabled: bool,
  #[serde(default)]
  pub stop_and_go_braking: bool,
  #[serde(default)]
  pub trailer_fill_limit: bool,
  #[serde(default)]
  pub automatic_motor_start_enabled: bool,
  #[serde(default)]
  pub growth_mode: i8,
  #[serde(default)]
  pub planned_days_per_period: i8,
  #[serde(default)]
  pub fruit_destruction: bool,
  #[serde(default)]
  pub plowing_required_enabled: bool,
  #[serde(default)]
  pub stones_enabled: bool,
  #[serde(default)]
  pub weeds_enabled: bool,
  #[serde(default)]
  pub lime_required: bool,
  #[serde(default)]
  pub is_snow_enabled: bool,
  #[serde(default)]
  pub fuel_usage: i8,
  #[serde(default)]
  pub helper_buy_fuel: bool,
  #[serde(default)]
  pub helper_buy_seeds: bool,
  #[serde(default)]
  pub helper_buy_fertilizer: bool,
  #[serde(default)]
  pub helper_slurry_source: i8,
  #[serde(default)]
  pub helper_manure_source: i8,
  #[serde(default)]
  pub density_map_revision: i8,
  #[serde(default)]
  pub terrain_texture_revision: i8,
  #[serde(default)]
  pub terrain_lod_texture_revision: i8,
  #[serde(default)]
  pub split_shapes_revision: i8,
  #[serde(default)]
  pub tip_collision_revision: i8,
  #[serde(default)]
  pub placement_collision_revision: i8,
  #[serde(default)]
  pub navigation_collision_revision: i8,
  #[serde(default)]
  pub map_density_map_revision: i8,
  #[serde(default)]
  pub map_terrain_texture_revision: i8,
  #[serde(default)]
  pub map_terrain_lod_texture_revision: i8,
  #[serde(default)]
  pub map_split_shapes_revision: i8,
  #[serde(default)]
  pub map_tip_collision_revision: i8,
  #[serde(default)]
  pub map_placement_collision_revision: i8,
  #[serde(default)]
  pub map_navigation_collision_revision: i8,
  pub disaster_destruction_state: Box<str>,
  #[serde(default)]
  pub dirt_interval: i8,
  #[serde(default)]
  pub time_scale: f32,
  #[serde(default)]
  pub auto_save_interval: f32,
  /// Introduced in Patch 1.14 (FS25)
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub is_cross_platform_savegame: bool,
  /// Introduced in Patch 1.18 (FS25)
  #[cfg(feature = "fs25")]
  pub initial_platform_name: Box<str>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
  #[serde(rename = "foundHelpIcons")]
  pub found_help_icons: Box<str>
}

/// Two other fields are not shown as they aren't visible in the payload.<br>
/// If you have such data, please kindly contribute.<br>
/// Remaining fields: `shownElements` and `shownHints`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntroductionHelp {
  #[serde(rename = "@active")]
  pub active: Box<str>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsgStatistics {
  pub money:     i32,
  #[serde(rename = "playTime")]
  pub play_time: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapsSplitShapeFileIds {
  #[serde(rename = "@count")]
  pub count: Box<str>,
  #[serde(rename = "id", default)]
  pub ids:   Vec<SplitShapeId>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitShapeId {
  #[serde(rename = "@id")]
  pub id: Box<str>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotSystem {
  #[serde(rename = "@slotUsage")]
  pub slot_usage: Box<str>
}

#[cfg(feature = "fs25")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoliageTypes {
  #[serde(rename = "foliageType")]
  pub foliages: Vec<FoliageType>
}

#[cfg(feature = "fs25")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoliageType {
  /// Lowercased name of foliage
  #[serde(rename = "@name")]
  pub name:     Box<str>,
  /// Path to foliage's XML file
  #[serde(rename = "@filename")]
  pub filename: Box<str>
}
