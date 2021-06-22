use crate::models::{HabitId, HabitName, HabitUnit};
use serde::__private::fmt::Error;
use serde::__private::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Habit {
    pub id: HabitId,
    pub name: HabitName,
    pub quantum: f64,
    pub unit: HabitUnit,
    pub notes: String,
    pub streak: Streak,
    pub difficulty: Difficulty,
    pub kind: Kind,
    pub active: bool,
}

impl std::fmt::Display for Habit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(
            f,
            "Habit:\n\tId:{:?}\n\tName:{:?}\n\tQuantum:{}\n\tUnit:{:?}\n\tActive:{}",
            self.id, self.name, self.quantum, self.unit, self.active
        )?;
        Ok(())
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq, Serialize, Deserialize)]
pub enum Streak {
    Daily,
    Weekly,
    Monthly
}
impl Streak {
    pub fn to_string(&self) -> &str {
        match *self {
            Streak::Daily => "Daily",
            Streak::Weekly => "Weekly",
            Streak::Monthly => "Monthly",
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Trivial,
    Easy,
    Medium,
    Hard
}
impl Difficulty {
    fn to_string(&self) -> &str {
        match *self {
            Difficulty::Trivial => "Trivial",
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}


#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq, Serialize, Deserialize)]
pub enum Kind {
    Positive,
    Negative
}
impl Kind {
    fn to_string(&self) -> &str {
        match *self {
            Kind::Positive => "Positive",
            Kind::Negative => "Negative",
        }
    }
}


impl Habit {
    // as well as to the methods...
    pub fn new(
        id: HabitId,
        name: HabitName,
        quantum: f64,
        unit: HabitUnit,
        streak: Option<Streak>,
        difficulty: Option<Difficulty>,
        kind: Option<Kind>,
        notes: Option<String>,
    ) -> Self {

        Self {
            id,
            name: name,
            quantum: if quantum.is_sign_positive() { quantum } else { 0. },
            unit: if unit.0.is_empty() { HabitUnit("unit".into()) } else { unit },
            streak: match streak {
                Some(s) => s,
                None => Streak::Daily
            },
            difficulty: match difficulty {
                Some(d) => d,
                None => Difficulty::Easy
            },
            kind: match kind {
                Some(h) => h,
                None => Kind::Positive
            },
            notes: match notes {
                Some(n) if n.len() > 280 => panic!("Habit's note cannot be longer than {} characters!", 280),
                Some(n) => n,
                None => "".into(),
            },
            active: true,
        }
    }
}

// Rust struct getters for enum fields
impl Habit {
    pub fn streak(&self) -> &Streak {
        &self.streak
    }
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
    pub fn difficulty(&self) -> &Difficulty {
        &self.difficulty
    }
}

// or, more getters
impl Habit {
    pub fn is_easy(&self) -> bool {
        match self.difficulty {
            Difficulty::Easy => true,
            _ => false
        }
    }
}

trait HasPosNeg {
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}

impl HasPosNeg for Habit {
    fn is_positive(&self) -> bool {
        match self.kind {
            Kind::Positive => true,
            Kind::Negative => false
        }
    }
    fn is_negative(&self) -> bool {
        !self.is_positive()
    }
}

impl Habit {
    pub fn activate(&mut self) {
        self.active = true
    }

    pub fn deactivate(&mut self) {
        self.active = false
    }

    pub fn toggle(&mut self) {
        self.active = !self.active
    }
}

#[derive(PartialEq, Debug)]
pub struct DeletedHabit(pub Habit);
