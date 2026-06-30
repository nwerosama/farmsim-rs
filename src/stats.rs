use {
  super::{
    DssMod,
    Mod,
    Validation
  },
  serde::{
    Deserialize,
    Deserializer,
    Serialize
  },
  serde_json::Value
};

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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server:   Option<Server>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub slots:    Option<Slots>,
  #[serde(default)]
  pub vehicles: Vec<Vehicle>,
  #[serde(deserialize_with = "serde_mods", default)]
  pub mods:     Vec<Mod>
}

impl Validation for DssData {
  fn is_valid(&self) -> bool { self.server.as_ref().is_some_and(|s| s.day_time > 0) }
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
  pub map_overview_filename: Box<str>,
  /// This value will always return zero in games beyond Farming Simulator 17<br>
  /// For servers post-FS17, use `money` field in [`CsgStatistics`] instead<br>
  ///
  /// [`CsgStatistics`]: super::careersavegame::CsgStatistics
  pub money:                 i32,
  pub name:                  String,
  pub version:               String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slots {
  pub capacity: i8,
  pub used:     i8,
  pub players:  Vec<DssPlayer>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Note: The `xyz` coordinates are only visible when player is on foot!
pub struct DssPlayer {
  #[serde(rename = "isUsed", skip_serializing_if = "Option::is_none")]
  pub is_used:  Option<bool>,
  #[serde(rename = "isAdmin", skip_serializing_if = "Option::is_none")]
  pub is_admin: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uptime:   Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub x:        Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub y:        Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub z:        Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name:     Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name:     Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub category: Option<String>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub type_:    Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub x:        Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub y:        Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub z:        Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fills:    Option<Vec<VehicleFill>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleFill {
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub type_: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub level: Option<f32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id:       Option<i32>,
  #[serde(rename = "isOwned", skip_serializing_if = "Option::is_none")]
  pub is_owned: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub x:        Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub z:        Option<f32>
}

// it is only present in XML data but not available in JSON data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Farmland {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name:  Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id:    Option<i8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub owner: Option<i8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub area:  Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub x:     Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub z:     Option<f32>
}
