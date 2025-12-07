#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_new_book() {
        let _book = Book::new("paradise lost", "milton", Some("97812345"));
    }
}
