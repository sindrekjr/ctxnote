use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct Note {
    pub author: String,
    pub email: String,
    pub time: DateTime<Utc>,
    pub text: String,
}

impl Note {
    pub fn new(author: String, email: String, text: String) -> Self {
        Self {
            author,
            email,
            text,
            time: Utc::now(),
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
        let email = &self.email;

        let timestamp = self.time.timestamp();

        let text = &self.text;

        format!("{} <{}> {} {}", author, email, timestamp, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_correctly_stores_text() {
        let author = "Tester";
        let email = "tester@testers.com";
        let text = "This is a test entry.";
        let note = Note::new(author.to_string(), email.to_string(), text.to_string());

        assert_eq!(note.text, text);
    }

    #[test]
    fn from_str_correctly_deserialize_string() {
        let author = "Tester";
        let email = "tester@testers.com";
        let timestamp = Utc::now().timestamp();
        let text = "This is a test entry.";

        let entry_str = format!("{} <{}> {} {}", author, email, timestamp, text);
        let parsed_note = Note::from_str(&entry_str);

        assert_eq!(parsed_note.author, author);
        assert_eq!(parsed_note.email, email);
        assert_eq!(parsed_note.time.timestamp(), timestamp);
        assert_eq!(parsed_note.text, text);
    }

    #[test]
    fn to_str_correctly_serialize_str() {
        let note = Note {
            author: String::from("Tester"),
            email: String::from("tester@testers.com"),
            time: Utc::now(),
            text: String::from("This is a test entry."),
        };

        assert_eq!(
            note.to_str(),
            format!(
                "{} <{}> {} {}",
                note.author,
                note.email,
                note.time.timestamp(),
                note.text
            )
        )
    }
}
