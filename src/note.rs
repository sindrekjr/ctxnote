use crate::conf::Config;
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct Note {
    pub author: String,
    pub email: Option<String>,
    pub time: DateTime<Utc>,
    pub text: String,
}

impl Note {
    pub fn new(text: String) -> Self {
        let config = Config::get();

        Self {
            author: match config.user.name {
                Some(name) => name,
                None => String::from("user"),
            },
            email: config.user.email,
            time: Utc::now(),
            text,
        }
    }

    pub fn from_str(entry_str: &str) -> Self {
        let mut chars = entry_str.chars();
        let author: String = chars.by_ref().take_while(|&c| c != '<').collect();
        let email: String = chars.by_ref().take_while(|&c| c != '>').collect();
        let timestamp: i64 = chars
            .by_ref()
            .skip(1)
            .take_while(|&c| c != ' ')
            .collect::<String>()
            .parse()
            .unwrap();
        let text: String = chars.collect();

        let author = author.trim().to_owned();
        let email = Some(email);
        let time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);

        Self {
            author,
            email,
            time,
            text,
        }
    }

    pub fn to_str(&self) -> String {
        let author = &self.author;
        let email = match &self.email {
            Some(e) => e,
            None => "",
        };

        let timestamp = self.time.timestamp();

        let text = &self.text;

        format!("{} <{}> {} {}", author, email, timestamp, text)
    }
}
