use crate::models::{ ValidationError };
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(PartialEq, Debug, Clone, Hash, Eq, Serialize, Deserialize)]
pub struct HabitUnit(pub String);

impl HabitUnit {
    pub fn new(unit: String) -> Result<HabitUnit, ValidationError> {
        if unit.len() > 15 {
            Err(ValidationError::new("Habit's unit cannot be longer than 15 characters!"))
        } else {
            Ok(HabitUnit(unit))
        }
    }
}

impl std::fmt::Display for HabitUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod habit_unit_tests {
    use crate::models::HabitUnit;
    use fake::{Fake};


    #[test]
    fn creating_a_unit_with_more_than_15chars_should_fail() {
        let new_unit = HabitUnit::new((16..50).fake::<String>());
        assert!(new_unit.is_err())
    }
}
