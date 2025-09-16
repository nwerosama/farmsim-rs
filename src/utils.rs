pub fn format_daytime(day_time: i32) -> String {
  let hours = day_time / 3_600_000;
  let mins = (day_time % 3_600_000) / 60_000;
  format!("{hours:02}:{mins:02}")
}

pub fn format_player_uptime(uptime: i32) -> String {
  let mins: i32;
  let mut hrs: i32 = 0;

  if uptime >= 60 {
    hrs = uptime / 60;
    mins = uptime % 60;
  } else {
    mins = uptime;
  }

  format!(
    "{}{}",
    if hrs > 0 { format!("{hrs} h ") } else { String::new() },
    if mins > 0 { format!("{mins} m") } else { String::new() }
  )
}
