use std::{collections::HashSet, fs, path::Path, str::FromStr};

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use uuid::uuid;

use spine::{Author, Book, Isbn, Library, Status};

#[test]
fn spine_add_adds_new_book_to_existing_library() {
    let out_path = Path::new("tests/data/spine.json");
    fs::copy("tests/data/single_book.json", out_path).unwrap();
    let mut expected = Library::new();
    expected.add(Book {
        id: uuid!("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"),
        title: "hadji murat".to_owned(),
        author: Author::from_str("leo tolstoy").unwrap(),
        isbn: Some(Isbn::from_str("9781847494818").unwrap()),
        status: Status::Read,
        tags: HashSet::from(["classic".into(), "russian".into()]),
        ..Default::default()
    });
    let mut book2 = Book {
        id: uuid!("b1b2b3b4-c1c2-d1d2-e1e2-e3e4e5e6e7e8"),
        title: "norwegian wood".to_owned(),
        author: Author::from_str("haruki murakami").unwrap(),
        status: Status::Reading,
        tags: HashSet::from(["japanese".into()]),
        ..Default::default()
    };

    let mut cmd = cargo_bin_cmd!("spine");

    #[rustfmt::skip]
    let assert = cmd
        .args([
            "add",
            "--reading",
            "--tag", "japanese",
            "norwegian wood",
            "haruki murakami",
        ])
        .current_dir("tests/data")
        .assert();

    assert
        .success()
        .append_context("main", "failed to add book")
        .stdout(predicate::str::contains("Book added!"))
        .append_context("main", "wrong output");

    let actual = Library::open(out_path).unwrap();
    book2.id = actual.all().last().expect("book in library").id.clone();
    expected.add(book2);
    assert_eq!(actual, expected);

    fs::remove_file(out_path).unwrap();
}
