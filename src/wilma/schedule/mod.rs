use serde::{Deserialize, Serialize};

pub mod reservation;
pub mod term;

pub use reservation::Reservation;
pub use term::Term;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Schedule {
    terms: Vec<Term>,
    // Maybe have some other system for this
    #[serde(rename(deserialize = "Schedule"))]
    reservations: Vec<Reservation>,
}
