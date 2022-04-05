use crate::wilma::user::UserType;

// todo
pub fn verify_url<T: AsRef<str>>(data: &T) -> String {
    format!("https://{}", data.as_ref())
}

/// A quick GET helper method to clean up code.
pub async fn get<T: AsRef<str>>(url: T) -> anyhow::Result<String> {
    let response_text = reqwest::get(url.as_ref()).await?.text().await?;

    Ok(response_text)
}

/// A utility for parsing formkeys. Returns a tuple that has a structure of (user_type, user_id,
/// session_id).
pub async fn parse_formkey(formkey: &str) -> (UserType, u32, String) {
    let mut iterator = formkey.split(":");

    let user_type = UserType::from(iterator.next().unwrap());
    let user_id: u32 = iterator.next().unwrap().parse().unwrap();
    let session_id = iterator.next().unwrap();

    (user_type, user_id, session_id.into())
}
