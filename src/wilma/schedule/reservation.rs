use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "u32")]
/// Weekday enum. One-based.
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl From<u32> for Weekday {
    fn from(data: u32) -> Self {
        use Weekday::*;

        match data {
            1 => Monday,
            2 => Tuesday,
            3 => Wednesday,
            4 => Thursday,
            5 => Friday,
            _ => panic!("Tried to build weekday enum from an invalid number (expected 1-5)."),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Reservation {
    /// The weekday the reservation is in.
    #[serde(rename(deserialize = "Day"))]
    weekday: Weekday,

    /// The classes from which students participating in the reservation are in.
    class: String,

    /// The color for the class used in Wilma's UI
    color: String,

    /// TODO: make a time struct
    end: String,

    // todo
    // groups: Vec<Group>
    /// The ID of the schedule.
    #[serde(rename(deserialize = "ScheduleID"))]
    id: u32,

    // todo: time struct
    start: String,
}

/*"Class": "8B/D/F/G/H",
  "Color": "#A6CAF0",
  "Day": 5,
  "End": "13:50",
  "Groups": [
    {
      "Caption": "vMVALATMEDIA.8VAL",
      "Class": "8B/D/F/G/H",
      "CourseId": 2548,
      "FullCaption": "Opiskelijan työvälineet ja mediataito",
      "Id": 61285,
      "Rooms": [
        {
          "Caption": "AT 2",
          "Id": 31,
          "LongCaption": "ATK-luokka",
          "ScheduleVisible": true
        }
      ],
      "ShortCaption": "vMVALATMEDIA.8VAL",
      "Teachers": [
        {
          "Caption": "KLP",
          "Id": 48,
          "LongCaption": "Lähteenmäki-Paukku Katja",
          "ScheduleVisible": true
        }
      ]
    }
  ],
  "ReservationID": 2859,
  "ScheduleID": 1213761319,
  "Start": "13:05",
  "X1": 40000,
  "X2": 50000,
  "Y1": 309,
  "Y2": 354
}*/
