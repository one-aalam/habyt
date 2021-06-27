extern crate chrono;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HabitLog {
    pub id: u32,
    pub quantum: f64,
    pub notes: String,
    pub date: NaiveDate,
}

impl HabitLog {
    // as well as to the methods...
    pub fn new(
        id: u32,
        quantum: f64,
        notes: Option<String>,
    ) -> Self {

        Self {
            id,
            quantum,
            notes: match notes {
                Some(n) if n.len() > 280 => panic!("Log's note cannot be longer than {} characters!", 280),
                Some(n) => n,
                None => "".into(),
            },
            date: Local::today().naive_local()
        }
    }
}
