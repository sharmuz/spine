use std::{fs, path::Path};

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

use spine::{Library, Status};

#[test]
fn spine_add_adds_new_book_to_existing_library() {
    let out_path = Path::new("tests/data/spine.json");
    fs::copy("tests/data/single_book.json", out_path).unwrap();
    let mut expected = Library::new();
    expected.add(
        "hadji murat",
        "leo tolstoy",
        Some("9781847494818"),
        Some(Status::Read),
    );
    expected.add(
        "norwegian wood",
        "haruki murakami",
        None,
        Some(Status::Reading),
    );

    let mut cmd = cargo_bin_cmd!("spine");

    let assert = cmd
        .args(["add", "--reading", "norwegian wood", "haruki murakami"])
        .current_dir("tests/data")
        .assert();

    assert
        .success()
        .append_context("main", "failed to add book")
        .stdout(predicate::str::contains("Book added!"))
        .append_context("main", "wrong output");

    let actual = Library::open(out_path).unwrap();
    assert_eq!(actual, expected);

    fs::remove_file(out_path).unwrap();
}
