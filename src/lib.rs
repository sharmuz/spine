use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display},
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
};

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    #[default]
    Want,
    Reading,
    Read,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Book {
    title: String,
    author: String,
    isbn: Option<String>,
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

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    #[must_use]
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    /// Adds a new book to the library.
    pub fn add(&mut self, title: &str, author: &str, isbn: Option<&str>, status: Option<Status>) {
        let book = Book::new(title, author, isbn, status.unwrap_or_default());
        self.books.push(book);
    }

    /// Shows all books in the library.
    #[must_use]
    pub fn show(&self) -> String {
        self.books
            .iter()
            .map(|b| format!("{b}"))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Saves the library to a file.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        let buf = BufWriter::new(file);

        serde_json::to_writer(buf, self)?;

        Ok(())
    }

    /// Opens the library from a file.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let buf = BufReader::new(file);
        let deserialized: Self = serde_json::from_reader(buf)?;

        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_new_creates_new_book() {
        let _book = Book::new("paradise lost", "milton", Some("97812345"), Status::Read);
    }

    #[test]
    fn test_add_adds_new_book_without_isbn() {
        let mut my_lib = Library::new();
        let my_book = Book {
            title: "the tale of genji".to_owned(),
            author: "murasaki shikibu".to_owned(),
            isbn: None,
            status: Status::Want,
        };
        let expected = Library {
            books: vec![my_book],
        };

        my_lib.add("the tale of genji", "murasaki shikibu", None, None);

        assert_eq!(my_lib, expected);
    }

    #[test]
    fn show_shows_all_books() {
        let mut my_lib = Library::new();
        my_lib.add("burmese days", "george orwell", None, None);
        my_lib.add(
            "kim",
            "rudyard kipling",
            Some("97812345"),
            Some(Status::Read),
        );
        let expected = "burmese days, george orwell\nkim, rudyard kipling";

        let show_all = my_lib.show();

        assert_eq!(show_all, expected);
    }

    #[test]
    fn save_then_open_restores_library() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("my_library.json");
        let mut my_lib = Library::new();
        my_lib.add("burmese days", "george orwell", None, None);
        my_lib.add(
            "kim",
            "rudyard kipling",
            Some("97812345"),
            Some(Status::Read),
        );

        my_lib.save(&file_path).unwrap();
        let opened = Library::open(&file_path).unwrap();

        assert_eq!(opened, my_lib, "wrong data");
    }
}
