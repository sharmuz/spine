use std::fs;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn spine_add_adds_new_book_to_existing_library() {
    fs::copy("tests/data/single_book.json", "tests/data/spine.json").unwrap();

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

    fs::remove_file("tests/data/spine.json").unwrap();
}
