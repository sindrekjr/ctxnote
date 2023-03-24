use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Note {
    pub author: String,
    pub email: String,
    pub time: DateTime<Utc>,
    pub text: String,
}

impl Note {
    pub fn new(author: String, email: String, text: String) -> Self {
        let time = Utc::now();

        Self {
            author,
            email,
            text,
            time,
        }
    }

    pub fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Note {
    pub fn from_storage_string(storage_str: &str) -> Option<Self> {
        let parts = storage_str.trim().split('§').collect::<Vec<&str>>();

        if parts.len() != 5 {
            return None;
        }

        let author = parts[1].to_owned();
        let email = parts[2].to_owned();
        let time = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(parts[3].parse().ok()?, 0),
            Utc,
        );
        let text = parts[4].to_owned();

        Some(Self {
            author,
            email,
            time,
            text,
        })
    }

    pub fn to_storage_string(&self) -> String {
        let time = self.time.timestamp();
        format!("{}§{}§{}§{}§{}", self.id(), self.author, self.email, time, self.text)
    }
}

impl Hash for Note {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.author.hash(state);
        self.time.hash(state);
        self.text.hash(state);
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "* {} ({}) {} - {}", self.id(), self.time, self.text, self.author,)
    }
}
