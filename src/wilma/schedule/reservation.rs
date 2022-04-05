use serde::de;
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
    /// The hour the time is at (for example, the time 13:50 is at hour 13).
    pub hours: u32,

    /// The minute the time is at (for example, the time 13:50 is at minute 50).
    pub minutes: u32,
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
pub struct Group {
    /// The caption of the group
    caption: String,

    class: String,

    course_id: u32,

    full_caption: String,

    id: u32,

    #[serde(default)]
    rooms: Vec<Room>,

    short_caption: String,

    teachers: Vec<Teacher>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Room {
    caption: String,
    id: u32,
    long_caption: String,
    schedule_visible: bool,
}

impl Room {
    /// The "abbreviation" (caption) of a room.
    pub fn caption(&self) -> &String {
        &self.caption
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    /// A more descriptive caption of the room.
    pub fn long_caption(&self) -> &String {
        &self.long_caption
    }

    /// Whether the room is visible on the schedule (I think!).
    pub fn schedule_visible(&self) -> bool {
        self.schedule_visible
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Teacher {
    pub caption: String,
    pub id: u32,
    pub long_caption: String,
    pub schedule_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Reservation {
    /// The weekday the reservation is in.
    #[serde(rename(deserialize = "Day"))]
    pub weekday: Weekday,

    /// The classes from which students participating in the reservation are in.
    pub class: String,

    /// The color for the class used in Wilma's UI
    pub color: Option<String>,

    /// The time when the reservation ends.
    #[serde(deserialize_with = "deserialize_time")]
    pub end: Time,

    /// Groups that are participating in the reservation.
    pub groups: Vec<Group>,

    /// The ID of the schedule the reservation is in.
    #[serde(rename(deserialize = "ScheduleID"))]
    pub id: u32,

    /// The ID of the reservation.
    #[serde(rename(deserialize = "ReservationID"))]
    pub reservation_id: u32,

    /// The time when the reservation starts.
    #[serde(deserialize_with = "deserialize_time")]
    pub start: Time,
}
