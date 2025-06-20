use serde::{
  Deserialize,
  Serialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Economy {
  pub great_demands: GreatDemands,
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
  pub unique_id:         String,
  #[serde(rename = "@fillTypeName")]
  pub fill_type_name:    String,
  #[serde(rename = "@demandMultiplier")]
  pub demand_multiplier: f32,
  #[serde(rename = "@demandStartDay")]
  pub demand_start_day:  i32,
  #[serde(rename = "@demandStartHour")]
  pub demand_start_hour: i32,
  #[serde(rename = "@demandDuration")]
  pub demand_duration:   i32,
  #[serde(rename = "@isRunning")]
  pub is_running:        String,
  #[serde(rename = "@isValid")]
  pub is_valid:          String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FillType {
  #[serde(rename = "@fillType")]
  pub fill_type:    String,
  #[serde(rename = "@totalAmount")]
  pub total_amount: Option<i32>,
  #[serde(default)]
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

impl std::fmt::Display for SeasonPeriod {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>
  ) -> std::fmt::Result {
    let (period, season) = match self {
      SeasonPeriod::EarlySpring => ("Early", "Spring"),
      SeasonPeriod::MidSpring => ("Mid", "Spring"),
      SeasonPeriod::LateSpring => ("Late", "Spring"),

      SeasonPeriod::EarlySummer => ("Early", "Summer"),
      SeasonPeriod::MidSummer => ("Mid", "Summer"),
      SeasonPeriod::LateSummer => ("Late", "Summer"),

      SeasonPeriod::EarlyAutumn => ("Early", "Autumn"),
      SeasonPeriod::MidAutumn => ("Mid", "Autumn"),
      SeasonPeriod::LateAutumn => ("Late", "Autumn"),

      SeasonPeriod::EarlyWinter => ("Early", "Winter"),
      SeasonPeriod::MidWinter => ("Mid", "Winter"),
      SeasonPeriod::LateWinter => ("Late", "Winter")
    };
    write!(f, "{period} {season}")
  }
}

pub fn calculate_economy_price(base_price: i32) -> i32 {
  let multiplier = 3.16f32;
  (base_price as f32 * multiplier).round() as i32
}
