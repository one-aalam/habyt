use crate::models::{ HabitName, HabitUnit, Streak, Difficulty, Kind };

#[derive(PartialEq, Debug, Clone)]
pub struct HabitDraft {
    pub name: HabitName,
    pub quantum: f64,
    pub unit: HabitUnit,
    pub streak: Option<Streak>,
    pub difficulty: Option<Difficulty>,
    pub kind: Option<Kind>,
    pub notes: Option<String>,
}

impl HabitDraft {
    // as well as to the methods...
    pub fn new(
        name: String,
        quantum: f64,
        unit: String,
    ) -> Self {

        Self {
            name: HabitName::new(name).expect("Habit name should exist"),
            quantum,
            unit: HabitUnit::new(unit).expect("Habit unit should exist"),
            streak: None,
            difficulty: None,
            kind: None,
            notes: None,
        }
    }

    pub fn from(
        name: String,
        quantum: f64,
        unit: String,
        streak: Option<Streak>,
        difficulty: Option<Difficulty>,
        kind: Option<Kind>,
        notes: Option<String>,
    ) -> Self {

        Self {
            name: HabitName::new(name).expect("Habit name should exist"),
            quantum,
            unit: HabitUnit::new(unit).expect("Habit unit should exist"),
            streak,
            difficulty,
            kind,
            notes,
        }
    }
}
