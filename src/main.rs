use spine::Library;

fn main() {
    let mut my_lib = Library::new();
    my_lib.add("hadji murat", "leo tolstoy", Some("9780123456789"), None);
    my_lib.add(
        "norwegian wood",
        "haruki murakami",
        None,
        Some(spine::Status::Read),
    );
    println!("Books in your library:\n\n{}", my_lib.show());
}
