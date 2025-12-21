use std::{fs, process::Command};

#[test]
fn spine_add_adds_new_book_to_existing_library() {
    fs::copy("tests/data/single_book.json", "tests/data/spine.json").unwrap();

    let cmd = Command::new("cargo")
        .args(["run", "--"])
        .args(["add", "--reading", "norwegian wood", "haruki murakami"])
        .current_dir("tests/data")
        .output()
        .expect("Failed to add book");

    assert!(cmd.status.success());
    assert_eq!(cmd.stdout, b"Book added!\n");

    fs::remove_file("tests/data/spine.json").unwrap();
}
