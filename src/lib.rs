use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
    path::Path,
    slice,
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

    /// Updates status of a book in the library.
    pub fn update_status(
        &mut self,
        search: (Option<&str>, Option<&str>, Option<&str>),
        new_status: Status,
    ) -> Result<(), io::Error> {
        let hits = self.search(search.0, search.1, search.2);
        if hits.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "No books found."));
        } else if hits.len() > 1 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Found multiple books. Please be more specific.",
            ));
        }
        let update_idx = self
            .books
            .iter()
            .position(|b| b == hits[0])
            .ok_or(io::Error::new(io::ErrorKind::Other, "No books found."))?;

        self.books[update_idx].status = new_status;
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

    /// Returns an iterator over all books in the library.
    #[must_use]
    pub fn all(&self) -> slice::Iter<'_, Book> {
        self.books.iter()
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
    use std::sync::LazyLock;
    use tempfile::tempdir;

    static BURMESE_DAYS: LazyLock<Book> = LazyLock::new(|| Book {
        title: "burmese days".to_owned(),
        author: "george orwell".to_owned(),
        ..Default::default()
    });
    static KIM: LazyLock<Book> = LazyLock::new(|| Book {
        title: "kim".to_owned(),
        author: "rudyard kipling".to_owned(),
        isbn: Some("9780199536467".to_owned()),
        status: Status::Read,
    });
    static EIGHTY_DAYS: LazyLock<Book> = LazyLock::new(|| Book {
        title: "around the world in eighty days".to_owned(),
        author: "jules verne".to_owned(),
        ..Default::default()
    });

    fn library_with_two_books() -> Library {
        let mut my_lib = Library::new();
        my_lib.add(BURMESE_DAYS.clone());
        my_lib.add(KIM.clone());
        my_lib
    }

    #[test]
    fn add_adds_new_book_without_isbn() {
        let mut my_lib = Library::new();

        my_lib.add(BURMESE_DAYS.clone());

        assert_eq!(my_lib.all().next().unwrap(), &*BURMESE_DAYS);
    }

    #[test]
    fn remove_removes_book_from_library() {
        let mut my_lib = library_with_two_books();

        my_lib.remove(Some("burmese"), None, None).unwrap();

        assert_ne!(my_lib.all().next().unwrap(), &*BURMESE_DAYS);
    }

    #[test]
    fn remove_throws_error_if_multiple_hits() {
        let mut my_lib = library_with_two_books();
        my_lib.add(EIGHTY_DAYS.clone());

        let err = my_lib.remove(Some("days"), None, None).unwrap_err();

        assert!(err.to_string().contains("Found multiple books."));
    }

    #[test]
    fn remove_throws_error_if_no_hits() {
        let mut my_lib = library_with_two_books();

        let err = my_lib.remove(Some("1984"), None, None).unwrap_err();

        assert!(err.to_string().contains("No books found."));
    }

    #[test]
    fn update_status_changes_book_status() {
        let mut my_lib = library_with_two_books();
        let expected = Book {
            status: Status::Reading,
            ..BURMESE_DAYS.clone()
        };

        my_lib.update_status((Some("burmese"), None, None), Status::Reading).unwrap();

        assert_eq!(my_lib.all().next().unwrap(), &expected);
    }

    #[test]
    fn update_status_throw_error_if_no_hit() {
        let mut my_lib = library_with_two_books();

        let err = my_lib.update_status((Some("1984"), None, None), Status::Reading).unwrap_err();

        assert!(err.to_string().contains("No books found."));
    }

    #[test]
    fn search_finds_single_hit_by_title() {
        let my_lib = library_with_two_books();

        let search_hits = my_lib.search(Some("burmese"), None, None);

        assert_eq!(search_hits, vec![&*BURMESE_DAYS]);
    }

    #[test]
    fn search_finds_multiple_hits_by_title() {
        let mut my_lib = library_with_two_books();
        my_lib.add(EIGHTY_DAYS.clone());

        let search_hits = my_lib.search(Some("days"), None, None);

        assert_eq!(search_hits, vec![&*BURMESE_DAYS, &*EIGHTY_DAYS]);
    }

    #[test]
    fn search_finds_multiple_hits_by_author() {
        let mut my_lib = library_with_two_books();
        let new_book = Book {
            title: "felix holt, the radical".to_owned(),
            author: "george eliot".to_owned(),
            isbn: None,
            status: Status::Want,
        };
        my_lib.add(new_book.clone());

        let search_hits = my_lib.search(None, Some("george"), None);

        assert_eq!(search_hits, vec![&*BURMESE_DAYS, &new_book]);
    }

    #[test]
    fn search_finds_single_hit_by_title_and_isbn() {
        let my_lib = library_with_two_books();

        let search_hits = my_lib.search(Some("kim"), None, Some("9780199536467"));

        assert_eq!(search_hits, vec![&*KIM]);
    }

    #[test]
    fn search_finds_nothing_by_title() {
        let my_lib = library_with_two_books();

        let search_hits = my_lib.search(Some("1984"), None, None);

        assert_eq!(search_hits, Vec::<&Book>::new());
    }

    #[test]
    fn search_finds_nothing_by_nothing() {
        let my_lib = library_with_two_books();

        let search_hits = my_lib.search(None, None, None);

        assert_eq!(search_hits, Vec::<&Book>::new());
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
}
