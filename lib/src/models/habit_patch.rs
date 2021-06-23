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

impl HabitPatch {
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
            name: if name.is_empty() { None } else { Some(HabitName::new(name).expect("Habit name must be valid")) },
            quantum: Some(quantum),
            unit: if unit.is_empty() { None } else { Some(HabitUnit::new(unit).expect("Habit unit must be valid")) },
            streak,
            difficulty,
            kind,
            notes,
        }
    }
}
