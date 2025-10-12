use std::fmt::Write;

pub fn fmt_daytime(daytime: i32) -> String {
  let hours = daytime / 3_600_000;
  let mins = (daytime % 3_600_000) / 60_000;

  let mut s = String::with_capacity(5);
  let _ = write!(&mut s, "{hours:02}:{mins:02}");
  s
}

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

#[test]
fn test_uptime() {
  assert_eq!(fmt_uptime(135), "2 h 15 m");
  assert_eq!(fmt_uptime(61), "1 h 1 m");
  assert_eq!(fmt_uptime(59), "59 m");
  assert_eq!(fmt_uptime(0), "0 m");
}

#[test]
fn test_daytime() {
  assert_eq!(fmt_daytime(9045683), "02:30");
  assert_eq!(fmt_daytime(413975), "00:06");
  assert_eq!(fmt_daytime(0), "00:00");
}
