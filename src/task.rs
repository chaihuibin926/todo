use serde::{Serialize, Deserialize};
use std::fmt;
use chrono::{Local, TimeZone, NaiveDate};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
pub struct Task {
  pub id: u32,
  pub content: String,
  pub done: bool,
  pub created_at: i64,
}

impl Task {
  pub fn new(content: String) -> Self {
    Self {
      id: 0,
      content,
      done: false,
      created_at: Local::now().timestamp(),
    }
  }

  pub fn date(&self) -> NaiveDate {
    Local.timestamp_opt(self.created_at, 0).unwrap().date_naive()
  }

  pub fn cmp_date(&self, temp: i64) -> Ordering {
    self.date()
      .cmp(
        &Local
          .timestamp_opt(temp, 0)
          .unwrap()
          .date_naive()
      )
  }
}

impl fmt::Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.done {
      const EMERALD_GREEN: &str = "\x1b[38;2;0;201;87m";
      const RESET: &str = "\x1b[0m";
      write!(f, "{}.[✅]{}{}{}", self.id, EMERALD_GREEN, self.content, RESET)
    } else {
      let now = Local::now().timestamp();
      if self.cmp_date(now) == Ordering::Less {
        const RED: &str = "\x1b[38;2;255;0;0m";
        const RESET: &str = "\x1b[0m";
        write!(f, "{}.[❌]{}{}{}", self.id, RED, self.content, RESET)
      } else {
        const YELLOW: &str = "\x1b[38;2;255;255;0m";
        const RESET: &str = "\x1b[0m";
        write!(f, "{}.[⏳]{}{}{}", self.id, YELLOW, self.content, RESET)
      }
    }
  }
}