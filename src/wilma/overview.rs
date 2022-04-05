use crate::wilma::{schedule::Reservation, Exam};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Overview {
    schedule: Vec<Reservation>,
    exams: Vec<Exam>,
}
