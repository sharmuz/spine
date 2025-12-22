use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
};

pub use crate::book::{Book, Status};

pub mod book;

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

    /// Searches library for books.
    pub fn search(
        &self,
        title: Option<&str>,
        _author: Option<&str>,
        _isbn: Option<&str>,
    ) -> Vec<&Book> {
        let mut hits: Vec<&Book> = Vec::new();
        if let Some(title) = title {
            let title_hits = self.books.iter().filter(|b| b.title.contains(title));
            hits.extend(title_hits);
        }
        hits
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
    fn add_adds_new_book_without_isbn() {
        let mut my_lib = Library::new();
        let my_book = Book::new("the tale of genji", "murasaki shikibu", None, Status::Want);
        let expected = Library {
            books: vec![my_book],
        };

        my_lib.add("the tale of genji", "murasaki shikibu", None, None);

        assert_eq!(my_lib, expected);
    }

    #[test]
    fn search_finds_single_match_by_title() {
        let my_lib = library_with_two_books();
        let my_book = Book::new("burmese days", "george orwell", None, Status::Want);
        let expected = vec![&my_book];

        let search_hits = my_lib.search(Some("burmese"), None, None);

        assert_eq!(search_hits, expected);
    }

    #[test]
    fn search_finds_multiple_matches_by_title() {
        let mut my_lib = library_with_two_books();
        my_lib.add("around the world in eighty days", "jules verne", None, None);
        let book_one = Book::new("burmese days", "george orwell", None, Status::Want);
        let book_two = Book::new(
            "around the world in eighty days",
            "jules verne",
            None,
            Status::Want,
        );
        let expected = vec![&book_one, &book_two];

        let search_hits = my_lib.search(Some("days"), None, None);

        assert_eq!(search_hits, expected);
    }

    #[test]
    fn show_shows_all_books() {
        let my_lib = library_with_two_books();
        let expected = "burmese days, george orwell\nkim, rudyard kipling";

        let show_all = my_lib.show();

        assert_eq!(show_all, expected);
    }

    #[test]
    fn save_then_open_restores_library() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("my_library.json");
        let my_lib = library_with_two_books();

        my_lib.save(&file_path).unwrap();
        let opened = Library::open(&file_path).unwrap();

        assert_eq!(opened, my_lib, "wrong data");
    }

    fn library_with_two_books() -> Library {
        let mut my_lib = Library::new();
        my_lib.add("burmese days", "george orwell", None, None);
        my_lib.add(
            "kim",
            "rudyard kipling",
            Some("97812345"),
            Some(Status::Read),
        );
        my_lib
    }
}
