use serde::de::{self, Unexpected};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

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

impl Weekday {
    pub fn fmt(&self) -> String {
        use Weekday::*;

        match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
        }
        .to_string()
    }

    pub fn fmt_finnish(&self) -> String {
        use Weekday::*;

        match self {
            Monday => "Maanantai",
            Tuesday => "Tiistai",
            Wednesday => "Keskiviikko",
            Thursday => "Torstai",
            Friday => "Perjantai",
        }
        .to_string()
    }
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

/// A time struct for creating abstractions around reservation start times.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    hours: u32,
    minutes: u32,
}

impl Time {
    /// Returns a formatted string. A time struct where `self.hours == 13` and `self.minutes == 50`
    /// would return "13:50".
    pub fn fmt(&self) -> String {
        let minutes_fmt = match self.minutes < 10 {
            true => format!("0{}", self.minutes),
            false => self.minutes.to_string(),
        };

        format!("{}:{}", self.hours, minutes_fmt)
    }

    /// The hour the time is at (for example, the time 13:50 is at hour 13).
    pub fn hours(&self) -> u32 {
        self.hours
    }

    /// The minute the time is at (for example, the time 13:50 is at minute 50).
    pub fn minutes(&self) -> u32 {
        self.minutes
    }
}

impl From<u32> for Time {
    fn from(data: u32) -> Self {
        let hours = (data as f32 / 60.0).floor();
        let time_hours_and_minutes = data as f32 / 60.0;

        let minutes = (time_hours_and_minutes - hours) * 60.0;

        Self {
            hours: hours as u32,
            minutes: minutes as u32,
        }
    }
}

impl From<&'_ str> for Time {
    fn from(data: &'_ str) -> Self {
        let (hours, minutes) = data.split_once(":").expect("Invalid time string provided.");

        Self {
            hours: hours.parse::<u32>().expect("Invalid time string provided."),
            minutes: minutes
                .parse::<u32>()
                .expect("Invalid time string provided."),
        }
    }
}

/// This is needed for deserializing time values (as they can either be a number or a string, thanks
/// Visva)
struct DeserializeTime;

impl<'de> de::Visitor<'de> for DeserializeTime {
    type Value = Time;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("u32 or string")
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Time::from(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Time::from(value))
    }
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<Time, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeTime)
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

    /// The time when the reservation ends.
    #[serde(deserialize_with = "deserialize_time")]
    end: Time,

    // todo
    // groups: Vec<Group>
    /// The ID of the schedule the reservation is in.
    #[serde(rename(deserialize = "ScheduleID"))]
    id: u32,

    /// The ID of the reservation.
    #[serde(rename(deserialize = "ReservationID"))]
    reservation_id: u32,

    /// The time when the reservation starts.
    #[serde(deserialize_with = "deserialize_time")]
    start: Time,
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
