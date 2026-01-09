use std::{
    fmt::{self, Display},
    io,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: Option<Isbn>,
    pub status: Status,
    pub tags: Vec<String>,
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.title, self.author)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Isbn {
    Isbn10(String),
    Isbn13(String),
}

impl Isbn {
    pub fn as_str(&self) -> &str {
        match self {
            Isbn::Isbn10(s) => s,
            Isbn::Isbn13(s) => s,
        }
    }
}

impl FromStr for Isbn {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.replace(" ", "").replace("-", "");
        let mut rev = norm.chars().rev();
        let last = rev.next();
        for c in rev {
            if !c.is_digit(10) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid ISBN: must contain only digits separated by hyphens or spaces.",
                ));
            }
        }
        if norm.len() == 10
            && (last.is_some_and(|c| c.is_digit(10) || c.eq_ignore_ascii_case(&'x')))
        {
            return Ok(Isbn::Isbn10(norm));
        }
        if norm.len() == 13
            && (&norm[..3] == "978" || &norm[..3] == "979")
            && last.is_some_and(|c| c.is_digit(10))
        {
            return Ok(Isbn::Isbn13(norm));
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid ISBN: must be either 10 digits or 13 digits with a prefix of 978/979.",
        ))
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    #[default]
    Want,
    Reading,
    Read,
}

impl FromStr for Status {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "want" => Ok(Status::Want),
            "reading" => Ok(Status::Reading),
            "read" => Ok(Status::Read),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid status: expected 'want', 'reading', or 'read'",
            )),
        }
    }
}
