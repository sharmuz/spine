use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    title: String,
    author: String,
    isbn: Option<String>,
}

impl Book {
    pub fn new(title: &str, author: &str, isbn: Option<&str>) -> Self {
        Self {
            title: title.to_owned(),
            author: author.to_owned(),
            isbn: isbn.map(String::from),
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.title, self.author)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    #[must_use]
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    pub fn add(&mut self, title: &str, author: &str, isbn: Option<&str>) {
        let book = Book::new(title, author, isbn);
        self.books.push(book);
    }

    #[must_use]
    pub fn show(&self) -> String {
        self.books
            .iter()
            .map(|b| format!("{b}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_new_book() {
        let _book = Book::new("paradise lost", "milton", Some("97812345"));
    }

    #[test]
    fn test_add_adds_new_book_without_isbn() {
        let mut my_lib = Library::new();
        let my_book = Book {
            title: "the tale of genji".to_owned(),
            author: "murasaki shikibu".to_owned(),
            isbn: None,
        };
        let expected = Library {
            books: vec![my_book],
        };

        my_lib.add("the tale of genji", "murasaki shikibu", None);

        assert_eq!(my_lib, expected);
    }

    #[test]
    fn show_shows_all_books() {
        let mut my_lib = Library::new();
        my_lib.add("1984", "george orwell", None);
        my_lib.add("kim", "rudyard kipling", Some("97812345"));
        let expected = "1984, george orwell\nkim, rudyard kipling";

        let show_all = my_lib.show();

        assert_eq!(show_all, expected);
    }
}
