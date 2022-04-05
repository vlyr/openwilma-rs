use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Group {
    id: u32,

    course_id: u32,

    course_name: String,

    #[serde(rename(deserialize = "CourseCode"))]
    code: String,

    name: String,

    caption: String,

    start_date: String,

    end_date: String,

    committed: bool,
}
