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

/// Converts the 'Growth Mode' integer to friendly name, e.g `Yes` when setting is on `1` in API
pub fn prettify_growth_mode(growth_mode: i8) -> &'static str {
  match growth_mode {
    1 => "Yes",
    2 => "No",
    3 => "Paused",
    _ => "Unknown"
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerSavegame {
  pub settings: Option<Settings>,
  pub map: Option<Map>,
  #[serde(rename = "introductionHelp")]
  pub introduction_help: Option<IntroductionHelp>,
  pub statistics: Option<CsgStatistics>,
  #[serde(rename = "mapsSplitShapeFileIds")]
  pub maps_split_shape_file_ids: Option<MapsSplitShapeFileIds>,
  #[serde(rename = "slotSystem")]
  pub slot_system: Option<SlotSystem>,
  #[cfg(feature = "fs25")]
  #[serde(rename = "foliageTypes")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
  pub savegame_name: String,
  pub creation_date: String,
  pub map_id: String,
  pub map_title: String,
  pub save_date_formatted: String,
  pub save_date: String,
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
  pub economic_difficulty: String,
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub has_initially_owned_farmlands: bool,
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub load_default_farm: bool,
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub start_with_guided_tour: bool,
  pub traffic_enabled: bool,
  pub stop_and_go_braking: bool,
  pub trailer_fill_limit: bool,
  pub automatic_motor_start_enabled: bool,
  pub growth_mode: i8,
  pub planned_days_per_period: i8,
  pub fruit_destruction: bool,
  pub plowing_required_enabled: bool,
  pub stones_enabled: bool,
  pub weeds_enabled: bool,
  pub lime_required: bool,
  pub is_snow_enabled: bool,
  pub fuel_usage: i8,
  pub helper_buy_fuel: bool,
  pub helper_buy_seeds: bool,
  pub helper_buy_fertilizer: bool,
  pub helper_slurry_source: i8,
  pub helper_manure_source: i8,
  pub density_map_revision: i8,
  pub terrain_texture_revision: i8,
  pub terrain_lod_texture_revision: i8,
  pub split_shapes_revision: i8,
  pub tip_collision_revision: i8,
  pub placement_collision_revision: i8,
  pub navigation_collision_revision: i8,
  pub map_density_map_revision: i8,
  pub map_terrain_texture_revision: i8,
  pub map_terrain_lod_texture_revision: i8,
  pub map_split_shapes_revision: i8,
  pub map_tip_collision_revision: i8,
  pub map_placement_collision_revision: i8,
  pub map_navigation_collision_revision: i8,
  pub disaster_destruction_state: String,
  pub dirt_interval: i8,
  pub time_scale: f32,
  pub auto_save_interval: f32,
  /// Recently introduced in Patch 1.14+
  #[cfg(feature = "fs25")]
  #[serde(default)]
  pub is_cross_platform_savegame: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
  #[serde(rename = "foundHelpIcons")]
  pub found_help_icons: String
}

/// Two other fields are not shown as they aren't visible in the payload.<br>
/// If you have such data, please kindly contribute.<br>
/// Remaining fields: `shownElements` and `shownHints`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntroductionHelp {
  #[serde(rename = "@active")]
  pub active: String
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
  pub count: String,
  #[serde(rename = "id", default)]
  pub ids:   Vec<SplitShapeId>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitShapeId {
  #[serde(rename = "@id")]
  pub id: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotSystem {
  #[serde(rename = "@slotUsage")]
  pub slot_usage: String
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
  pub name:     String,
  /// Path to foliage's XML file
  #[serde(rename = "@filename")]
  pub filename: String
}
