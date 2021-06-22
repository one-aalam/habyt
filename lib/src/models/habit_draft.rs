use crate::models::{HabitName, HabitUnit};
use crate::models::{ Streak, Difficulty, Kind }

#[derive(PartialEq, Debug, Clone)]
/// The content of the ticket, not yet saved in the [TicketStore](TicketStore::create).
pub struct TicketDraft {
    pub name: HabitName,
    pub quantum: f64,
    pub unit: HabitUnit,
    pub streak: Option<Streak>,
    pub difficulty: Option<Difficulty>,
    pub kind: Option<Kind>
    pub notes: Option<String>,
}
