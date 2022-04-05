use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Exam {
    /// The ID of SOMETHING.
    pub id: u32,

    /// The ID of the exam.
    pub exam_id: u32,

    /// The course the exam is from.
    pub course: String,

    /// The ID of the course the exam is from.
    pub course_id: u32,

    /// Will be none if the exam hasn't been given a name yet.
    pub name: Option<String>,

    /// The title of the course the exam is from.
    pub course_title: String,

    /// The grade that has been given from the exam.
    /// Will be Option::None if the user has not received a grade from the exam yet.
    pub grade: Option<String>,

    /// An array of teachers that the exam is from.
    pub teachers: Vec<Teacher>,

    /// The date that the exam will be had in.
    pub date: String,
}

/// A teacher struct.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Teacher {
    /// The ID of the teacher.
    #[serde(rename(deserialize = "TeacherId"))]
    pub id: u32,

    /// The name of the teacher.
    #[serde(rename(deserialize = "TeacherName"))]
    pub name: String,

    /// An abbreviation of the teacher's name. Usually the teacher's initials.
    #[serde(rename(deserialize = "TeacherCode"))]
    pub code: String,
}
