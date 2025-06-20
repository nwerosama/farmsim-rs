use {
  super::{
    DssMod,
    Mod
  },
  serde::{
    Deserialize,
    Deserializer,
    Serialize
  },
  serde_json::Value
};

// Used for the doc comment without making it look ugly.
#[allow(unused)]
use crate::careersavegame::Statistics;

// todo; make xml-compatible with same structs below but feature flag toggle?
//       as the fields below is all from JSON payload

fn serde_mods<'de, D>(deserializer: D) -> Result<Vec<Mod>, D::Error>
where
  D: Deserializer<'de>
{
  let raw_value: Value = Value::deserialize(deserializer)?;
  let mods: Vec<DssMod> = serde_json::from_value(raw_value).map_err(serde::de::Error::custom)?;
  Ok(mods.into_iter().map(Mod::DssFormat).collect())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DssData {
  pub server:   Option<Server>,
  pub slots:    Option<Slots>,
  pub vehicles: Vec<Vehicle>,
  #[serde(deserialize_with = "serde_mods", default)]
  pub mods:     Vec<Mod>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
  #[serde(rename = "dayTime")]
  pub day_time:              i32,
  pub game:                  String,
  #[serde(rename = "mapName")]
  pub map_name:              String,
  #[serde(rename = "mapSize")]
  pub map_size:              i32,
  #[serde(rename = "mapOverviewFilename")]
  pub map_overview_filename: String,
  /// This value will always return zero in games beyond Farming Simulator 17<br>
  /// For servers post-FS17, use `money` field in [Statistics] instead
  pub money:                 i32,
  pub name:                  String,
  pub version:               String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slots {
  pub capacity: i8,
  pub used:     i8,
  pub players:  Vec<Player>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Note: The `xyz` coordinates are only visible when player is on foot!
pub struct Player {
  #[serde(rename = "isUsed")]
  pub is_used:  Option<bool>,
  #[serde(rename = "isAdmin")]
  pub is_admin: Option<bool>,
  pub uptime:   Option<i32>,
  pub x:        Option<f32>,
  pub y:        Option<f32>,
  pub z:        Option<f32>,
  pub name:     Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
  pub name:     Option<String>,
  pub category: Option<String>,
  pub x:        Option<f32>,
  pub y:        Option<f32>,
  pub z:        Option<f32>,
  pub fills:    Option<Vec<VehicleFill>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleFill {
  #[serde(rename = "type")]
  pub type_: Option<String>,
  pub level: Option<f32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
  pub id:       Option<i32>,
  #[serde(rename = "isOwned")]
  pub is_owned: Option<bool>,
  pub x:        Option<f32>,
  pub z:        Option<f32>
}
