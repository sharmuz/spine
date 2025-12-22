use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    #[default]
    Want,
    Reading,
    Read,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Book {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) isbn: Option<String>,
    status: Status,
}

impl Book {
    pub fn new(title: &str, author: &str, isbn: Option<&str>, status: Status) -> Self {
        Self {
            title: title.to_owned(),
            author: author.to_owned(),
            isbn: isbn.map(String::from),
            status,
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.title, self.author)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_new_book() {
        let _book = Book::new("paradise lost", "milton", Some("97812345"), Status::Read);
    }
}
