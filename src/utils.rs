use std::fmt::Write;

/// Formats the server's daytime into "09:30", without quotes
pub fn fmt_daytime(daytime: i32) -> String {
  let hours = daytime / 3_600_000;
  let mins = (daytime % 3_600_000) / 60_000;

  let mut s = String::with_capacity(4);
  let _ = write!(&mut s, "{hours:02}:{mins:02}");
  s
}

/// Formats the uptime into "2 h 36 m", without quotes
pub fn fmt_uptime(uptime: i32) -> String {
  let mins = uptime % 60;
  let hrs = uptime / 60;

  let mut s = String::new();

  if hrs > 0 {
    let _ = write!(s, "{hrs} h");
    if mins > 0 {
      let _ = write!(s, " {mins} m");
    }
  } else {
    let _ = write!(s, "{mins} m");
  }

  s
}

/// Formats the uptime into "2 h 36 m", without quotes
///
/// This function delegates to [`fmt_uptime`] internally
pub fn fmt_uptime_ms(uptime: f32) -> String {
  fmt_uptime(uptime as i32 / 60_000)
}

/// Converts the 'Growth Mode' integer to friendly name, e.g `Yes` when setting
/// is on `1` in API
pub fn prettify_growth_mode(growth_mode: i8) -> &'static str {
  match growth_mode {
    1 => "Yes",
    2 => "No",
    3 => "Paused",
    _ => "Unknown"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_uptime() {
    assert_eq!(fmt_uptime(135), "2 h 15 m");
    assert_eq!(fmt_uptime(61), "1 h 1 m");
    assert_eq!(fmt_uptime(59), "59 m");
    assert_eq!(fmt_uptime(0), "0 m");
  }

  #[test]
  fn test_uptime_ms() {
    assert_eq!(fmt_uptime_ms(110667000.0), "30 h 44 m");
    assert_eq!(fmt_uptime_ms(92308600.0), "25 h 38 m");
    assert_eq!(fmt_uptime_ms(5002027.4), "1 h 23 m");
    assert_eq!(fmt_uptime_ms(0.0), "0 m");
  }

  #[test]
  fn test_daytime() {
    assert_eq!(fmt_daytime(39090413), "10:51");
    assert_eq!(fmt_daytime(29920000), "08:18");
    assert_eq!(fmt_daytime(9045683), "02:30");
    assert_eq!(fmt_daytime(0), "00:00");
  }

  #[test]
  fn test_growth_mode() {
    assert_eq!(prettify_growth_mode(1), "Yes");
    assert_eq!(prettify_growth_mode(2), "No");
    assert_eq!(prettify_growth_mode(3), "Paused");
    assert_eq!(prettify_growth_mode(4), "Unknown");
  }
}
