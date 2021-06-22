use crate::models::{ HabitName, HabitUnit, Streak, Difficulty, Kind };

#[derive(PartialEq, Debug, Clone)]
pub struct HabitPatch {
    pub name: Option<HabitName>,
    pub quantum: Option<f64>,
    pub unit: Option<HabitUnit>,
    pub streak: Option<Streak>,
    pub difficulty: Option<Difficulty>,
    pub kind: Option<Kind>,
    pub notes: Option<String>,
}
