/// A user type enum. Check each field to check out what they represent.
#[derive(Debug, Clone)]
pub enum UserType {
    /// Teacher
    Teacher,

    /// Student
    Student,

    /// School personnel
    Personnel,

    /// Parent of the student
    Guardian,

    /// Workplace instructor
    Instructor,

    /// Management
    Management,

    /// Wilma Account (this type means that account has to choose a role before continuing, for example if the account's type is guardian with multiple students "owned" by it)
    Passwd,

    /// Unknown
    Unknown,
}

impl<T: AsRef<str>> From<T> for UserType {
    fn from(data: T) -> Self {
        use UserType::*;

        match data.as_ref() {
            "teacher" => Teacher,
            "student" => Student,
            "personnel" => Personnel,
            "guardian" => Guardian,
            "instructor" => Instructor,
            "management" => Management,
            "passwd" => Passwd,
            _ => Unknown,
        }
    }
}

/// User/profile struct
/// https://github.com/OpenWilma/parsing/wiki/Profile-Details
#[derive(Debug, Clone)]
pub struct User {
    name: String,
    school: String,
    formkey: String,
    user_type: UserType,
    user_id: String,
}

impl User {
    pub fn new(name: String, school: String, formkey: String) -> Self {
        let mut formkey_data = formkey.split(":");
        let user_type = UserType::from(formkey_data.next().unwrap());
        let user_id = formkey_data.next().unwrap().into();

        Self {
            name,
            school,
            formkey,
            user_type,
            user_id,
        }
    }

    /// A formkey is like a CSRF token, but only changes when logging in and out.
    /// It has the structure `user_type:user_id:session_token/key`.
    pub fn formkey(&self) -> &String {
        &self.formkey
    }

    /// The user's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The school the user goes to.
    pub fn school(&self) -> &String {
        &self.school
    }

    /// The type of the user account. See more in the UserType enum.
    pub fn user_type(&self) -> &UserType {
        &self.user_type
    }

    pub fn user_id(&self) -> &String {
        &self.user_id
    }
}
