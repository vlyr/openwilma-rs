// soon
use crate::parser;
use crate::wilma::User;
use crate::Error as WilmaError;
use crate::{utils, wilma::IndexResponse};
use serde::{Deserialize, Serialize};
use serde_json::{from_str as string_to_json, Value};
use std::collections::HashMap;
use std::sync::Arc;

use reqwest::{
    cookie::{Cookie, Jar},
    redirect::Policy,
    Url,
};

/// A struct for grouping all of the credentials together.
/// Username: Used for representing the username on the Wilma server.
/// Password: Password used for the Wilma account.
/// Server: The Wilma server, turku.inschool.fi, for example.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Credentials<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub server: &'a str,
}

/// The main layer for interacting with Wilma.
/// Documentation is a work-in-progress.
pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    pub async fn login(credentials: Credentials<'_>) -> anyhow::Result<Self> {
        let http_builder = reqwest::Client::builder().redirect(Policy::none());
        let http = http_builder.build()?;

        let mut url = utils::verify_url(&credentials.server);

        let index_path = format!("{}/index_json", url);

        // Retrieving a session ID from https://WILMA_SERVER/index_json.
        let index_response: IndexResponse = string_to_json(&utils::get(&index_path).await?)?;

        let session_id = index_response.session_id();

        let mut login_info: HashMap<&str, &str> = HashMap::new();
        login_info.insert("Login", &credentials.username);
        login_info.insert("Password", &credentials.password);
        login_info.insert("SESSIONID", session_id);
        login_info.insert("CompleteJson", "");

        let login_path = format!("{}/login", url);

        let login_response = http
            .post(login_path)
            .form(&login_info)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        let cookies: Vec<Cookie> = login_response.cookies().collect();

        // OK for now.
        let cookie = cookies
            .iter()
            .find(|c| c.name() == "Wilma2SID")
            .ok_or(WilmaError::InvalidCredentials)?;

        let cookie_jar = Arc::new(Jar::default());
        let cookie_url = url.clone().parse::<Url>()?;

        let cookie_string = format!("Wilma2SID={}", cookie.value());

        cookie_jar.add_cookie_str(&cookie_string, &cookie_url);

        // Re-initiate the client with the Wilma2SID cookie.
        let builder = reqwest::Client::builder().redirect(Policy::none());
        let client = builder.cookie_provider(cookie_jar).build()?;

        // Get an "identity" string which is appended to the base URL.
        let res = client.get(url.clone()).send().await?.text().await?;
        let identity = parser::core::parse_identity(&res);

        let appended_url = format!("{}/{}", url, identity);

        Ok(Self {
            base_url: appended_url,
            http: client,
        })
    }

    pub async fn get_user_profile(&self) -> anyhow::Result<User> {
        // Sending a GET request to the index gives you a page with your profile information.
        let response = self
            .http
            .get(self.base_url.clone())
            .send()
            .await?
            .text()
            .await?;

        use parser::user as parser;

        let name = parser::parse_name(&response);
        let school = parser::parse_school(&response);
        let formkey = parser::parse_formkey(&response);

        Ok(User::new(name, school, formkey))
    }
}
