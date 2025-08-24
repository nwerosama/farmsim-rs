use serde::{
  Deserialize,
  Serialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
  pub day_time:              f32,
  pub current_day:           i32,
  pub current_monotonic_day: i32,
  pub real_hour_timer:       String,
  pub days_per_period:       i32
}

pub fn get_current_month(current_day: u32) -> String {
  let month_index = current_day % 12;
  match month_index {
    1 => "March".to_string(),
    2 => "April".to_string(),
    3 => "May".to_string(),
    4 => "June".to_string(),
    5 => "July".to_string(),
    6 => "August".to_string(),
    7 => "September".to_string(),
    8 => "October".to_string(),
    9 => "November".to_string(),
    10 => "December".to_string(),
    11 => "January".to_string(),
    12 => "February".to_string(),
    _ => format!("[INVALID_MONTH: {} - Index: {month_index}]", current_day.clone())
  }
}
