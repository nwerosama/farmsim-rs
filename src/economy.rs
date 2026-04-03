use {
  serde::{
    Deserialize,
    Serialize
  },
  std::fmt
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Economy {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub great_demands: Option<GreatDemands>,
  pub fill_types:    FillTypes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreatDemands {
  #[serde(rename = "greatDemand", default)]
  pub great_demand: Vec<GreatDemand>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FillTypes {
  #[serde(rename = "fillType", default)]
  pub fill_type: Vec<FillType>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreatDemand {
  #[serde(rename = "@uniqueId")]
  pub unique_id:         Box<str>,
  #[serde(rename = "@fillTypeName")]
  pub fill_type_name:    Box<str>,
  #[serde(rename = "@demandMultiplier")]
  pub demand_multiplier: f32,
  #[serde(rename = "@demandStartDay")]
  pub demand_start_day:  i32,
  #[serde(rename = "@demandStartHour")]
  pub demand_start_hour: i32,
  #[serde(rename = "@demandDuration")]
  pub demand_duration:   i32,
  #[serde(rename = "@isRunning")]
  pub is_running:        Box<str>,
  #[serde(rename = "@isValid")]
  pub is_valid:          Box<str>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FillType {
  #[serde(rename = "@fillType")]
  pub fill_type:    Box<str>,
  #[serde(rename = "@totalAmount", skip_serializing_if = "Option::is_none")]
  pub total_amount: Option<i32>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub history:      Option<History>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
  #[serde(rename = "period", default)]
  pub periods: Vec<Period>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
  #[serde(rename = "@period")]
  pub period: SeasonPeriod,
  #[serde(rename = "#text")]
  pub value:  i32
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeasonPeriod {
  #[default]
  EarlySpring,
  MidSpring,
  LateSpring,

  EarlySummer,
  MidSummer,
  LateSummer,

  EarlyAutumn,
  MidAutumn,
  LateAutumn,

  EarlyWinter,
  MidWinter,
  LateWinter
}

impl fmt::Display for SeasonPeriod {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>
  ) -> fmt::Result {
    match self {
      SeasonPeriod::EarlySpring => "Early Spring",
      SeasonPeriod::MidSpring => "Mid Spring",
      SeasonPeriod::LateSpring => "Late Spring",

      SeasonPeriod::EarlySummer => "Early Summer",
      SeasonPeriod::MidSummer => "Mid Summer",
      SeasonPeriod::LateSummer => "Late Summer",

      SeasonPeriod::EarlyAutumn => "Early Autumn",
      SeasonPeriod::MidAutumn => "Mid Autumn",
      SeasonPeriod::LateAutumn => "Late Autumn",

      SeasonPeriod::EarlyWinter => "Early Winter",
      SeasonPeriod::MidWinter => "Mid Winter",
      SeasonPeriod::LateWinter => "Late Winter"
    }
    .fmt(f)
  }
}

pub fn calculate_economy_price(base_price: i32) -> i32 {
  let multiplier = 3.16f32;
  (base_price as f32 * multiplier).round() as i32
}
