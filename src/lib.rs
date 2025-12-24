use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
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
    pub fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    /// Removes a book from the library
    pub fn remove(
        &mut self,
        title: Option<&str>,
        author: Option<&str>,
        isbn: Option<&str>,
    ) -> Result<(), io::Error> {
        let hits = self.search(title, author, isbn);
        if hits.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "No books found."));
        } else if hits.len() > 1 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Found multiple books. Please be more specific.",
            ));
        }

        let rm_idx = self
            .books
            .iter()
            .position(|b| b == hits[0])
            .ok_or(io::Error::new(io::ErrorKind::Other, "No books found."))?;

        self.books.remove(rm_idx);
        Ok(())
    }

    /// Searches library for books.
    pub fn search(
        &self,
        title: Option<&str>,
        author: Option<&str>,
        isbn: Option<&str>,
    ) -> Vec<&Book> {
        match (title, author, isbn) {
            (None, None, None) => Vec::new(),
            (_, _, _) => self
                .books
                .iter()
                .filter(|&b| {
                    title.is_none_or(|t| b.title.contains(t))
                        & author.is_none_or(|a| b.author.contains(a))
                        & isbn.is_none_or(|c| b.isbn.as_ref().is_some_and(|i| i.contains(c)))
                })
                .collect(),
        }
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
    fn remove_removes_book_from_library() {
        let mut my_lib = library_with_two_books();
        let mut expected = Library::new();
        expected.add("burmese days", "george orwell", None, None);

        my_lib.remove(Some("kim"), None, None).unwrap();

        assert_eq!(my_lib, expected);
    }

    #[test]
    fn remove_throws_error_if_multiple_matches() {
        let mut my_lib = library_with_two_books();
        my_lib.add("around the world in eighty days", "jules verne", None, None);
        
        let err = my_lib.remove(Some("days"), None, None).unwrap_err();

        assert!(err.to_string().contains("Found multiple books."));
    }

    #[test]
    fn remove_throws_error_if_no_matches() {
        let mut my_lib = library_with_two_books();
        
        let err = my_lib.remove(Some("1984"), None, None).unwrap_err();

        assert!(err.to_string().contains("No books found."));
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
    fn search_finds_multiple_matches_by_author() {
        let mut my_lib = library_with_two_books();
        my_lib.add("felix holt, the radical", "george eliot", None, None);
        let book_one = Book::new("burmese days", "george orwell", None, Status::Want);
        let book_two = Book::new(
            "felix holt, the radical",
            "george eliot",
            None,
            Status::Want,
        );
        let expected = vec![&book_one, &book_two];

        let search_hits = my_lib.search(None, Some("george"), None);

        assert_eq!(search_hits, expected);
    }

    #[test]
    fn search_finds_single_match_by_isbn() {
        let my_lib = library_with_two_books();
        let my_book = Book::new(
            "kim",
            "rudyard kipling",
            Some("9780199536467"),
            Status::Read,
        );

        let expected = vec![&my_book];

        let search_hits = my_lib.search(None, None, Some("9780199536467"));

        assert_eq!(search_hits, expected);
    }

    #[test]
    fn search_finds_single_match_by_title_and_isbn() {
        let my_lib = library_with_two_books();
        let my_book = Book::new(
            "kim",
            "rudyard kipling",
            Some("9780199536467"),
            Status::Read,
        );

        let expected = vec![&my_book];

        let search_hits = my_lib.search(Some("kim"), None, Some("9780199536467"));

        assert_eq!(search_hits, expected);
    }

    #[test]
    fn search_finds_nothing_by_title() {
        let my_lib = library_with_two_books();
        let expected: Vec<&Book> = Vec::new();

        let search_hits = my_lib.search(Some("1984"), None, None);

        assert_eq!(search_hits, expected);
    }

    #[test]
    fn search_finds_nothing_by_nothing() {
        let my_lib = library_with_two_books();
        let expected: Vec<&Book> = Vec::new();

        let search_hits = my_lib.search(None, None, None);

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
            Some("9780199536467"),
            Some(Status::Read),
        );
        my_lib
    }
}
