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
  pub real_hour_timer:       Box<str>,
  pub days_per_period:       i32
}

pub fn get_current_month(current_day: u32) -> Box<str> {
  let month_index = current_day % 12;
  match month_index {
    1 => "March".into(),
    2 => "April".into(),
    3 => "May".into(),
    4 => "June".into(),
    5 => "July".into(),
    6 => "August".into(),
    7 => "September".into(),
    8 => "October".into(),
    9 => "November".into(),
    10 => "December".into(),
    11 => "January".into(),
    12 => "February".into(),
    _ => format!("[INVALID_MONTH: {current_day} - Index: {month_index}]").into()
  }
}
