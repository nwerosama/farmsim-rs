use {
  super::{
    CsgMod,
    Mod
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
  pub settings: Option<Settings>,
  pub map: Option<Map>,
  #[serde(rename = "introductionHelp")]
  pub introduction_help: Option<IntroductionHelp>,
  pub statistics: Option<Statistics>,
  #[serde(rename = "mapsSplitShapeFileIds")]
  pub maps_split_shape_file_ids: Option<MapsSplitShapeFileIds>,
  #[serde(rename = "slotSystem")]
  pub slot_system: Option<SlotSystem>,
  #[serde(rename = "mod", deserialize_with = "serde_mods", default)]
  pub mods: Vec<Mod>
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
  pub auto_save_interval: f32
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
pub struct Statistics {
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
