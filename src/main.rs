use std::{error::Error, path::Path};

use spine::Library;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("spine.json");

    let my_lib= if path.exists() {
        Library::open(&path)?
    } else {
        let mut my_lib = Library::new();
        my_lib.add("hadji murat", "leo tolstoy", Some("9780123456789"), None);
        my_lib.add(
            "norwegian wood",
            "haruki murakami",
            None,
            Some(spine::Status::Read),
        );
        my_lib.save(&path)?;
        my_lib
    };
    println!("Books in your library:\n\n{}", my_lib.show());
    Ok(())
}
