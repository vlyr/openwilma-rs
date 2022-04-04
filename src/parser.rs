use utils::*;

pub mod profile {
    use super::utils::*;
    use scraper::{Html, Selector};

    /// Documentation in progress.
    pub fn parse_name(document: &str) -> String {
        let mut lines = document.split("\n");

        let profile_line = filter_line("class=\"teacher\"", &mut lines).unwrap();

        let fragment = Html::parse_fragment(profile_line);
        let selector = Selector::parse("span").unwrap();
        let element = fragment.select(&selector).next().unwrap();
        let child = element.children().next().unwrap();

        let text = child.value().as_text().unwrap().to_string();

        text
    }

    /// Documentation in progress.
    pub fn parse_formkey(document: &str) -> String {
        let mut lines = document.split("\n");

        let line = filter_line("formkey", &mut lines).unwrap();

        let fragment = Html::parse_fragment(line);
        let selector = Selector::parse("input").unwrap();
        let element = fragment.select(&selector).next().unwrap();

        element.value().attr("value").unwrap().into()
    }

    /// Documentation in progress.
    pub fn parse_school(document: &str) -> String {
        let mut lines = document.split("\n");

        let line = filter_line("class=\"school\"", &mut lines).unwrap();

        let fragment = Html::parse_fragment(line);
        let selector = Selector::parse("span").unwrap();
        let element = fragment.select(&selector).next().unwrap();
        let child = element.children().next().unwrap();

        let text = child.value().as_text().unwrap().to_string();

        text
    }
}

pub mod core {
    use super::utils::*;
    use scraper::{Html, Selector};

    /// Documentation in progress.
    pub fn parse_identity(document: &str) -> String {
        let mut lines = document.split("\n");

        let line = filter_line("text-style-link", &mut lines).unwrap();

        let fragment = Html::parse_fragment(line);
        let selector = Selector::parse("a").unwrap();
        let stuff = fragment.select(&selector).next().unwrap();

        let mut identity = stuff.value().attr("href").unwrap().to_string();
        identity.remove(0);
        identity
    }
}

mod utils {
    pub fn filter_line<'a, I>(pattern: &str, lines: &mut I) -> Option<&'a str>
    where
        I: Iterator<Item = &'a str>,
    {
        lines.find(|l| l.contains(pattern))
    }
}
