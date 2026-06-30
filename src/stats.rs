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
  serde_json::Value,
  std::num::{
    NonZeroI8,
    NonZeroUsize
  }
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
  pub server:        Option<Server>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub slots:         Option<Slots>,
  #[serde(default)]
  pub vehicles:      Vec<Vehicle>,
  #[serde(deserialize_with = "serde_mods", default)]
  pub mods:          Vec<Mod>,
  #[serde(default)]
  pub fields:        Vec<Field>,
  /// Only present if `&idcode=` is passed to the URL
  pub configuration: Configuration,
  /// Only present if `&idcode=` is passed to the URL
  pub statistics:    DssStatistics
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
  pub map_size:              i16,
  #[serde(rename = "mapOverviewFilename")]
  pub map_overview_filename: Box<str>,
  /// This value will always return zero in games beyond Farming Simulator 17.
  ///
  /// For servers post-FS17, use [`CsgStatistics::money`] field instead.
  ///
  /// [`CsgStatistics::money`]: super::careersavegame::CsgStatistics::money
  pub money:                 i64,
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

/// It is only present in XML data but not available in JSON data
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
  /// Game name, e.g. `Farming Simulator 25 (1.20.0.0)`
  pub game:                Box<str>,
  /// Server name, e.g. `Dedi Server`
  #[serde(rename = "serverName")]
  pub server_name:         Box<str>,
  /// Game password, e.g. `ABCD1234`
  #[serde(rename = "gamePassword", skip_serializing_if = "Option::is_none")]
  pub game_password:       Option<Box<str>>,
  /// Server language, e.g. `en`
  #[serde(rename = "gameLanguage")]
  pub game_language:       Box<str>,
  /// Savegame index, e.g. `1`
  #[serde(rename = "savegameIndex")]
  pub savegame_index:      NonZeroI8,
  /// Server's IP address, e.g. `172.16.0.1`
  pub ip:                  Box<str>,
  /// Server's UDP port, e.g. `10823`
  pub port:                NonZeroUsize,
  /// Player capacity, e.g. `8`
  pub slots:               NonZeroI8,
  /// Savegame's economic difficulty level, e.g. `2`
  pub difficulty:          NonZeroI8,
  /// Autosave interval in minutes, e.g. `15`
  #[serde(rename = "saveInterval")]
  pub save_interval:       NonZeroUsize,
  /// Web API interval in seconds, e.g. `360`
  #[serde(rename = "webStatsInterval")]
  pub web_stats_interval:  NonZeroUsize,
  /// Pause server if there's nobody playing, e.g. `1`
  #[serde(rename = "pauseGameIfEmpty")]
  pub pause_game_if_empty: NonZeroI8,
  /// Whether server should appear on multiplayer list for consoles, e.g. `true`
  #[serde(rename = "crossPlay")]
  pub crossplay:           bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DssStatistics {
  /// Graph data of Host CPU usage (30-min interval)
  #[serde(rename = "cpuUsage")]
  pub cpu_usage:    Box<DssUsageData>,
  /// Donut data of memory usage in bytes
  #[serde(rename = "memoryUsage")]
  pub memory_usage: Box<DssUsage>,
  /// Donut data of disk usage in bytes
  #[serde(rename = "diskUsage")]
  pub disk_usage:   Box<DssUsage>,
  /// Server uptime in milliseconds
  pub uptime:       f32,
  /// Graph data of players (24-hour interval)
  pub players:      Box<DssUsageData>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DssUsageData {
  interval: NonZeroUsize,
  data:     Vec<f32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DssUsage {
  max:     NonZeroUsize,
  current: NonZeroUsize
}
