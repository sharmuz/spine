use std::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    #[default]
    Want,
    Reading,
    Read,
}

impl FromStr for Status {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "want" => Ok(Status::Want),
            "reading" => Ok(Status::Reading),
            "read" => Ok(Status::Read),
            _ => Err("Invalid status: expected 'want', 'reading', or 'read'"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub status: Status,
    pub tags: Vec<String>,
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.title, self.author)
    }
}
