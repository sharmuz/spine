use std::{error::Error, path::Path};

use clap::{Parser, Subcommand};

use spine::Library;

#[derive(Parser)]
#[command(name = "spine")]
#[command(about = "Your personal command-line librarian!", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show your books
    Show,
    /// Add a new book
    Add {
        #[arg(required = true)]
        title: String,
        #[arg(required = true)]
        author: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let path = Path::new("spine.json");
    let mut my_lib = if path.exists() {
        Library::open(&path)?
    } else {
        Library::new()
    };

    match args.command {
        Commands::Show => {
            println!("Books in your library:\n\n{}", my_lib.show());
        }
        Commands::Add { title, author } => {
            my_lib.add(&title, &author, None, None);
            my_lib.save(&path)?;
            println!("Book added!")
        }
    }

    Ok(())
}
