use {
  serde::{
    Deserialize,
    Serialize
  },
  std::num::NonZeroUsize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PropertyState {
  None,
  Owned,
  Leased,
  Mission,
  ShopConfig
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicles {
  pub vehicle: Vec<SgVehicle>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgVehicle {
  #[serde(rename = "@filename")]
  pub filename:       Box<str>,
  #[serde(rename = "@uniqueId")]
  pub unique_id:      Box<str>,
  #[serde(rename = "@age")]
  pub age:            f32,
  #[serde(rename = "@price")]
  pub price:          f32,
  #[serde(rename = "@farmId")]
  pub farm_id:        usize,
  #[serde(rename = "@propertyState")]
  pub property_state: Box<PropertyState>,
  #[serde(rename = "@operatingTime")]
  pub operating_time: f32,
  #[serde(rename = "component", default)]
  pub components:     Vec<SgVehicleComponent>,
  #[serde(rename = "configuration", default)]
  pub configurations: Vec<SgVehicleConfiguration>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wheels:         Option<SgVehicleWheels>,
  #[serde(default)]
  pub washable:       Vec<SgWashable>,
  #[serde(default)]
  pub wearable:       Vec<SgWearable>,
  #[serde(rename = "licensePlates", skip_serializing_if = "Option::is_none")]
  pub license_plates: Option<SgLicensePlates>,
  #[serde(rename = "fillUnit", skip_serializing_if = "Option::is_none")]
  pub fill_unit:      Option<SgFillUnit>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub drivable:       Option<SgDrivable>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgVehicleComponent {
  #[serde(rename = "@index")]
  pub index:    NonZeroUsize,
  #[serde(rename = "@position")]
  pub position: Box<str>,
  #[serde(rename = "@rotation")]
  pub rotation: Box<str>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgVehicleConfiguration {
  #[serde(rename = "@name")]
  pub name:      Box<str>,
  #[serde(rename = "@id")]
  pub id:        Box<str>,
  #[serde(rename = "@isActive")]
  pub is_active: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgWashable {
  #[serde(rename = "dirtNode")]
  pub dirt_nodes: Vec<SgDirtNode>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgDirtNode {
  #[serde(rename = "@amount")]
  pub amount:     f32,
  #[serde(rename = "@wetness")]
  pub wetness:    f32,
  #[serde(rename = "@snowScale", skip_serializing_if = "Option::is_none")]
  pub snow_scale: Option<f32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgWearable {
  #[serde(rename = "@damage")]
  pub damage:     f32,
  #[serde(rename = "wearNode")]
  pub wear_nodes: Vec<SgWearableNode>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgWearableNode {
  #[serde(rename = "@amount")]
  pub amount: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgVehicleWheels {
  #[serde(rename = "@lastConfigId")]
  pub last_config_id: Box<str>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgLicensePlates {
  #[serde(rename = "@variation")]
  pub variation:       NonZeroUsize,
  #[serde(rename = "@characters")]
  pub characters:      Box<str>,
  #[serde(rename = "@colorIndex")]
  pub color_index:     NonZeroUsize,
  #[serde(rename = "@placementIndex")]
  pub placement_index: NonZeroUsize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgFillUnit {
  #[serde(rename = "unit")]
  pub units: Vec<SgFillUnits>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgFillUnits {
  #[serde(rename = "@index")]
  pub index:      usize,
  #[serde(rename = "@fillType")]
  pub fill_type:  Box<str>,
  #[serde(rename = "@fillLevel")]
  pub fill_level: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SgDrivable {
  #[serde(rename = "@cruiseControl")]
  pub cruise_control:         usize,
  #[serde(rename = "@cruiseControlReverse")]
  pub cruise_control_reverse: usize,
  #[serde(rename = "@odometerMilage")]
  pub odometer_milage:        f32
}
