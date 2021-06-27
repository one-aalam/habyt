use crate::models::{DeletedHabit, Habit, HabitDraft, HabitId, HabitLog, HabitPatch};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct HabitStore {
    current_id: u32,
    data: HashMap<HabitId, Habit>,
}

#[derive(Serialize, Deserialize)]
pub struct HabitLogStore {
    current_id: u32,
    data: HashMap<u32, HabitLog>,
}

impl HabitStore {
    pub fn new() -> Self {
        Self {
            current_id: 0,
            data: HashMap::new(),
        }
    }

    pub fn create(&mut self, draft: HabitDraft) -> HabitId {
        let id = self.generate_id();
        let habit = Habit::new(
            id,
            draft.name,
            draft.quantum,
            draft.unit,
            draft.streak,
            draft.difficulty,
            draft.kind,
            draft.notes,
        );
        self.data.insert(habit.id, habit);
        id
    }

    pub fn delete(&mut self, id: HabitId) -> Option<DeletedHabit> {
        self.data.remove(&id).map(DeletedHabit)
    }

    pub fn list(&self) -> Vec<&Habit> {
        self.data.iter().map(|(_, habit)| habit).collect()
    }

    fn generate_id(&mut self) -> HabitId {
        self.current_id += 1;
        self.current_id
    }

    pub fn get(&self, id: HabitId) -> Option<&Habit> {
        self.data.get(&id)
    }

    pub fn update(&mut self, id: HabitId, patch: HabitPatch) -> Option<()> {
        self.data.get_mut(&id).map(|h| {
            if let Some(name) = patch.name {
                if !name.0.is_empty() {
                    h.name = name;
                }
            }
            if let Some(quantum) = patch.quantum {
                if quantum > 0. {
                    h.quantum = quantum;
                }
            }
            if let Some(unit) = patch.unit {
                h.unit = unit;
            }
            if let Some(streak) = patch.streak {
                h.streak = streak;
            }
            if let Some(difficulty) = patch.difficulty {
                h.difficulty = difficulty;
            }
            if let Some(kind) = patch.kind {
                h.kind = kind;
            }
            if let Some(notes) = patch.notes {
                h.notes = notes;
            }
        })
    }
}

impl HabitLogStore {
    pub fn new() -> Self {
        Self {
            current_id: 0,
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, log: HabitLog) -> u32 {
        let id = self.generate_id();
        self.data.insert(id, log);
        id
    }

    fn generate_id(&mut self) -> u32 {
        self.current_id += 1;
        self.current_id
    }

    pub fn get(&self, id: u32) -> Option<&HabitLog> {
        self.data.get(&id)
    }

    pub fn list(&self) -> Vec<&HabitLog> {
        self.data.iter().map(|(_, log_entry)| log_entry).collect()
    }

    pub fn get_log_by_habit(&self) -> HashMap<u32, Vec<(NaiveDate, f64)>> {
        let mut log = HashMap::new();

        for (_, log_entry) in self.data.iter() {
            if !log.contains_key(&log_entry.id) {
                log.insert(log_entry.id, vec![]);
            }
            log.get_mut(&log_entry.id)
                .unwrap()
                .push((log_entry.date, log_entry.quantum));
        }
        log
    }

    fn get_log_entry(&self, habit_id: &HabitId) -> Option<(&u32, &HabitLog)> {
        return self
            .data
            .iter()
            .find(|(_, log_entry)| log_entry.id == *habit_id);
    }
}

#[cfg(test)]
mod habit_store_tests {
    use crate::models::{
        DeletedHabit, Habit, HabitDraft, HabitId, HabitName, HabitPatch, HabitUnit,
    };
    use crate::store::HabitStore;
    use fake::{Fake, Faker};

    #[test]
    fn can_create_habit_with_provided_details() {
        //arrange
        let draft = HabitDraft::new((3..25).fake::<String>(), 700.0, (3..15).fake::<String>());
        let mut habit_store = HabitStore::new();

        //act
        let habit_id = habit_store.create(draft.clone());

        //assert
        let habit = habit_store
            .get(habit_id)
            .expect("Failed to retrieve habit.");
        assert_eq!(habit.name, draft.name);
        assert_eq!(habit.quantum, draft.quantum);
        assert_eq!(habit.unit, draft.unit);
    }

    #[test]
    fn can_delete_habit_with_provided_id() {
        //arrange
        let draft = HabitDraft::new((3..25).fake::<String>(), 700.0, (3..15).fake::<String>());
        let mut habit_store = HabitStore::new();
        let habit_id = habit_store.create(draft.clone());

        //act
        let deleted_habit = habit_store
            .delete(habit_id)
            .expect("There was no habit to delete.");

        //assert
        assert_eq!(deleted_habit.0.id, habit_id);
        assert_eq!(habit_store.get(habit_id), None);
    }

    #[test]
    fn deleting_a_habit_that_does_not_exist_returns_none() {
        //arrange
        let mut habit_store = HabitStore::new();

        //act
        let deleted_habit = habit_store.delete(Faker.fake());

        //assert
        assert_eq!(deleted_habit, None);
    }

    #[test]
    fn listing_habits_of_an_empty_store_returns_an_empty_collection() {
        // Arrange
        let habit_store = HabitStore::new();

        // Act
        let habits = habit_store.list();

        // Assert
        assert!(habits.is_empty())
    }

    // #[test]
    // fn listing_habits_should_return_them_all() {
    //     // Arrange
    //     let mut habit_store = HabitStore::new();
    //     let n_habits = Faker.fake::<u16>() as usize;
    //     let habits: HashSet<_> = (0..n_habits)
    //         .map(|_| create_habit_in_store(&mut habit_store))
    //         .collect();

    //     // Act
    //     let retrieved_habits = habit_store.list();

    //     // Assert
    //     assert_eq!(retrieved_habits.len(), n_habits);
    //     let retrieved_habits: HashSet<_> = retrieved_habits
    //         .into_iter()
    //         .map(|h| h.to_owned())
    //         .collect();
    //     assert_eq!(habits, retrieved_habits);
    // }

    fn create_habit_in_store(store: &mut HabitStore) -> &Habit {
        // arrange
        let draft = HabitDraft::new((3..25).fake::<String>(), 700.0, (3..15).fake::<String>());
        let habit_id = store.create(draft);
        store
            .get(habit_id)
            .expect("Failed to retrieve habit")
            .to_owned()
    }

    // #[test]
    // fn updating_habit_info_via_patch_should_update_habit() {
    //     // arrange
    //     let mut habit_store = HabitStore::new();

    //     let habit = create_habit_in_store(&mut habit_store);

    //     let patch = HabitPatch {
    //         name: Some(HabitName::new((3..25).fake::<String>()).expect("Failed to get the name")),
    //         quantum: Some(650.0),
    //         unit: Some(HabitUnit::new((3..15).fake::<String>()).expect("Failed to get the unit")),
    //         streak: None,
    //         difficulty: None,
    //         kind: None,
    //         notes: None,
    //     };

    //     let expected = patch.clone();

    //     //act
    //     habit_store.update_habit(habit.id, patch);
    //     let updated_habit = habit_store
    //         .get(habit.id)
    //         .expect("Failed to retrieve habit.");

    //     //assert
    //     assert_eq!(updated_habit.name, expected.name.expect("Failed to get the name"));
    //     assert_eq!(updated_habit.quantum, expected.quantum);
    //     assert_eq!(updated_habit.unit, expected.unit.expect("Failed to get the unit"));
    // }
}
