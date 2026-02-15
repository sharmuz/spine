use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
    path::Path,
    slice,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::book::{Author, Book, Isbn, Status};

pub mod book;
pub mod cli;
pub mod tui;

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
    pub fn add(&mut self, mut book: Book) {
        if book.id.is_nil() {
            book.id = Uuid::new_v4();
        }
        self.books.push(book);
    }

    /// Removes a book from the library
    pub fn remove(&mut self, id: Uuid) -> Result<(), io::Error> {
        let rm_idx = self.get_index(id)?;
        self.books.remove(rm_idx);

        Ok(())
    }

    /// Updates status of a book in the library.
    pub fn update_status(&mut self, id: Uuid, new_status: Status) -> Result<(), io::Error> {
        let update_idx = self.get_index(id)?;
        self.books[update_idx].status = new_status;

        Ok(())
    }

    pub fn tag<I>(&mut self, id: Uuid, tags: I) -> Result<(), io::Error>
    where
        I: IntoIterator<Item = String>,
    {
        let tag_idx = self.get_index(id)?;
        self.books[tag_idx].tags.extend(tags);

        Ok(())
    }

    pub fn untag(&mut self, id: Uuid, tags: &Vec<String>) -> Result<(), io::Error> {
        let tag_idx = self.get_index(id)?;
        self.books[tag_idx].tags.retain(|t| !tags.contains(t));

        Ok(())
    }

    fn get_index(&self, id: Uuid) -> Result<usize, io::Error> {
        self.books
            .iter()
            .position(|b| b.id == id)
            .ok_or_else(|| io::Error::other("No books found."))
    }

    /// Searches library for books.
    #[must_use]
    pub fn search(&self, search: &LibrarySearch) -> impl Iterator<Item = &Book> {
        self.books.iter().filter(|&b| {
            search.title.as_ref().is_none_or(|t| b.title.contains(t))
                && search
                    .author
                    .as_ref()
                    .is_none_or(|a| b.author.to_string().contains(a))
                && search
                    .isbn
                    .as_ref()
                    .is_none_or(|c| b.isbn.as_ref().is_some_and(|i| i.as_str().contains(c)))
                && search.status.is_none_or(|s| b.status == s)
                && search
                    .tags
                    .as_ref()
                    .is_none_or(|ts| ts.iter().all(|t| b.tags.contains(t)))
        })
    }

    /// Returns an iterator over all books in the library.
    pub fn all(&self) -> slice::Iter<'_, Book> {
        self.books.iter()
    }

    /// Saves the library to a file.
    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
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
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let buf = BufReader::new(file);
        let deserialized: Self = serde_json::from_reader(buf)?;

        Ok(deserialized)
    }
}

#[derive(Clone, Debug, Default)]
pub struct LibrarySearch {
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
    pub status: Option<Status>,
    pub tags: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, str::FromStr, sync::LazyLock};
    use tempfile::tempdir;
    use uuid::uuid;

    static BURMESE_DAYS: LazyLock<Book> = LazyLock::new(|| Book {
        id: uuid!("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"),
        title: "burmese days".to_owned(),
        author: Author::from_str("george orwell").unwrap(),
        ..Default::default()
    });
    static KIM: LazyLock<Book> = LazyLock::new(|| Book {
        id: uuid!("b1b2b3b4-c1c2-d1d2-e1e2-e3e4e5e6e7e8"),
        title: "kim".to_owned(),
        author: Author::from_str("rudyard kipling").unwrap(),
        isbn: Some(Isbn::from_str("9780199536467").unwrap()),
        status: Status::Read,
        tags: HashSet::from(["1800s".into(), "classic".into()]),
        ..Default::default()
    });
    static EIGHTY_DAYS: LazyLock<Book> = LazyLock::new(|| Book {
        id: uuid!("c1c2c3c4-d1d2-e1e2-f1f2-f3f4f5f6f7f8"),
        title: "around the world in eighty days".to_owned(),
        author: Author::from_str("jules verne").unwrap(),
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
        let rm_id = BURMESE_DAYS.id;

        my_lib.remove(rm_id).unwrap();

        assert_ne!(my_lib.all().next().unwrap(), &*BURMESE_DAYS);
    }

    #[test]
    fn remove_throws_error_if_id_not_present() {
        let mut my_lib = library_with_two_books();
        let rm_id = uuid!("c1c2c3c4-d1d2-e1e2-f1f2-f3f4f5f6f7f8");

        let err = my_lib.remove(rm_id).unwrap_err();

        assert!(err.to_string().contains("No books found."));
    }

    #[test]
    fn update_status_changes_book_status() {
        let mut my_lib = library_with_two_books();
        let expected = Book {
            status: Status::Reading,
            ..BURMESE_DAYS.clone()
        };
        let update_id = BURMESE_DAYS.id;

        my_lib.update_status(update_id, Status::Reading).unwrap();

        assert_eq!(my_lib.all().next().unwrap(), &expected);
    }

    #[test]
    fn update_status_throws_error_if_id_not_present() {
        let mut my_lib = library_with_two_books();
        let update_id = uuid!("c1c2c3c4-d1d2-e1e2-f1f2-f3f4f5f6f7f8");

        let err = my_lib
            .update_status(update_id, Status::Reading)
            .unwrap_err();

        assert!(err.to_string().contains("No books found."));
    }

    #[test]
    fn tag_adds_first_tag_to_book() {
        let mut my_lib = library_with_two_books();
        let expected = Book {
            tags: HashSet::from(["british-raj".into()]),
            ..BURMESE_DAYS.clone()
        };

        my_lib
            .tag(BURMESE_DAYS.id, vec!["british-raj".into()])
            .unwrap();

        assert_eq!(my_lib.all().next().unwrap(), &expected);
    }

    #[test]
    fn tag_adds_additional_tags_to_book() {
        let mut my_lib = library_with_two_books();
        let expected = Book {
            tags: HashSet::from([
                "1800s".into(),
                "british-raj".into(),
                "classic".into(),
                "spy".into(),
            ]),
            ..KIM.clone()
        };

        my_lib
            .tag(
                KIM.id,
                vec!["spy".into(), "british-raj".into(), "1800s".into()],
            )
            .unwrap();

        assert_eq!(my_lib.all().last().unwrap(), &expected);
    }

    #[test]
    fn untag_removes_existing_tags() {
        let mut my_lib = library_with_two_books();
        let expected = Book {
            tags: HashSet::from(["classic".into()]),
            ..KIM.clone()
        };

        my_lib
            .untag(KIM.id, &vec!["1800s".into(), "illustrated".into()])
            .unwrap();

        assert_eq!(my_lib.all().last().unwrap(), &expected);
    }

    #[test]
    fn get_index_returns_correct_index() {
        let mut my_lib = library_with_two_books();
        my_lib.add(EIGHTY_DAYS.clone());
        let id = uuid!("c1c2c3c4-d1d2-e1e2-f1f2-f3f4f5f6f7f8");

        let index = my_lib.get_index(id).unwrap();

        assert_eq!(index, 2);
    }

    #[test]
    fn search_finds_single_hit_by_title() {
        let my_lib = library_with_two_books();
        let my_search = LibrarySearch {
            title: Some("burmese".into()),
            ..Default::default()
        };

        let search_hits: Vec<_> = my_lib.search(&my_search).collect();

        assert_eq!(search_hits, vec![&*BURMESE_DAYS]);
    }

    #[test]
    fn search_finds_multiple_hits_by_title() {
        let mut my_lib = library_with_two_books();
        my_lib.add(EIGHTY_DAYS.clone());
        let my_search = LibrarySearch {
            title: Some("days".into()),
            ..Default::default()
        };

        let search_hits: Vec<_> = my_lib.search(&my_search).collect();

        assert_eq!(search_hits, vec![&*BURMESE_DAYS, &*EIGHTY_DAYS]);
    }

    #[test]
    fn search_finds_multiple_hits_by_author() {
        let mut my_lib = library_with_two_books();
        let new_book = Book {
            id: uuid!("d1d2d3d4-e1e2-f1f2-a1a2-a3a4a5a6a7a8"),
            title: "felix holt, the radical".to_owned(),
            author: Author::from_str("george eliot").unwrap(),
            isbn: None,
            status: Status::Want,
            ..Default::default()
        };
        my_lib.add(new_book.clone());
        let my_search = LibrarySearch {
            author: Some("george".into()),
            ..Default::default()
        };

        let search_hits: Vec<_> = my_lib.search(&my_search).collect();

        assert_eq!(search_hits, vec![&*BURMESE_DAYS, &new_book]);
    }

    #[test]
    fn search_finds_single_hit_by_title_and_isbn() {
        let my_lib = library_with_two_books();
        let my_search = LibrarySearch {
            title: Some("kim".into()),
            isbn: Some("9780199536467".into()),
            ..Default::default()
        };

        let search_hits: Vec<_> = my_lib.search(&my_search).collect();

        assert_eq!(search_hits, vec![&*KIM]);
    }

    #[test]
    fn search_finds_single_hit_by_status() {
        let my_lib = library_with_two_books();
        let my_search = LibrarySearch {
            status: Some(Status::Read),
            ..Default::default()
        };

        let search_hits: Vec<_> = my_lib.search(&my_search).collect();

        assert_eq!(search_hits, vec![&*KIM]);
    }

    #[test]
    fn search_finds_single_hit_by_tags() {
        let my_lib = library_with_two_books();
        let my_search = LibrarySearch {
            tags: Some(vec!["1800s".into(), "classic".into()]),
            ..Default::default()
        };

        let search_hits: Vec<_> = my_lib.search(&my_search).collect();

        assert_eq!(search_hits, vec![&*KIM]);
    }

    #[test]
    fn search_finds_nothing_by_title() {
        let my_lib = library_with_two_books();
        let my_search = LibrarySearch {
            title: Some("1984".into()),
            ..Default::default()
        };

        let search_hits: Vec<_> = my_lib.search(&my_search).collect();

        assert!(search_hits.is_empty());
    }

    #[test]
    fn search_finds_nothing_by_tags() {
        let my_lib = library_with_two_books();
        let my_search = LibrarySearch {
            tags: Some(vec!["1800s".into(), "japanese".into()]),
            ..Default::default()
        };

        let search_hits: Vec<_> = my_lib.search(&my_search).collect();

        assert!(search_hits.is_empty());
    }

    #[test]
    fn search_finds_all_by_nothing() {
        let my_lib = library_with_two_books();

        let search_hits: Vec<_> = my_lib
            .search(&LibrarySearch {
                ..Default::default()
            })
            .collect();

        assert_eq!(search_hits, my_lib.all().collect::<Vec<_>>());
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
