use crate::models::ValidationError;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(PartialEq, Debug, Clone, Hash, Eq, Serialize, Deserialize)]
pub struct HabitName(pub String);

impl HabitName {
    pub fn new(name: String) -> Result<HabitName, ValidationError> {
        if name.is_empty() {
            Err(ValidationError::new("Habit's Name cannot be empty!"))
        } else if name.trim().split_whitespace().count() > 3 {
            Err(ValidationError::new(
                "Habit's name cannot have more than 3 words!!",
            ))
        } else if name.len() > 50 {
            Err(ValidationError::new(
                "Habit's name cannot be longer than 50 characters!",
            ))
        } else {
            Ok(HabitName(name))
        }
    }
}

impl std::fmt::Display for HabitName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod habit_name_tests {
    use crate::models::HabitName;
    use fake::Fake;

    #[test]
    fn creating_an_empty_name_should_fail() {
        let new_name = HabitName::new("".to_string());
        assert!(new_name.is_err())
    }

    #[test]
    fn creating_a_name_with_more_than_three_words_should_fail() {
        let new_name = HabitName::new("habit name has more than four words".to_string());
        assert!(new_name.is_err())
    }

    #[test]
    fn creating_a_name_with_more_than_50chars_should_fail() {
        let new_name = HabitName::new((51..100).fake::<String>());
        assert!(new_name.is_err())
    }
}
