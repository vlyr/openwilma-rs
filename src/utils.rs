// todo
pub fn verify_url<T: AsRef<str>>(data: &T) -> String {
    format!("https://{}", data.as_ref())
}

/// A quick GET helper method to clean up code.
pub async fn get<T: AsRef<str>>(url: T) -> anyhow::Result<String> {
    let response_text = reqwest::get(url.as_ref()).await?.text().await?;

    Ok(response_text)
}
