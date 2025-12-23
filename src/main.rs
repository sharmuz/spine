use std::{error::Error, path::Path};

use clap::{Args, Parser, Subcommand};

use spine::{Library, Status};

#[derive(Parser)]
#[command(name = "spine")]
#[command(about = "spine is your personal command-line librarian!", long_about = None)]
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
        title: String,

        author: String,

        #[arg(short, long)]
        isbn: Option<String>,

        #[arg(long, group = "status")]
        want: bool,

        #[arg(long, group = "status")]
        reading: bool,

        #[arg(long, group = "status")]
        read: bool,
    },

    // Remove an existing book
    Remove(RemoveArgs),
}

#[derive(Args)]
#[group(required = true, multiple = true)]
struct RemoveArgs {
    #[arg(short, long)]
    title: Option<String>,

    #[arg(short, long)]
    author: Option<String>,

    #[arg(short, long)]
    isbn: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let path = Path::new("spine.json");
    let mut my_lib = if path.exists() {
        Library::open(path)?
    } else {
        Library::new()
    };

    match args.command {
        Commands::Show => {
            println!("Books in your library:\n\n{}", my_lib.show());
        }
        Commands::Add {
            title,
            author,
            isbn,
            reading,
            read,
            ..
        } => {
            let status = if read {
                Status::Read
            } else if reading {
                Status::Reading
            } else {
                Status::Want
            };
            my_lib.add(&title, &author, isbn.as_deref(), Some(status));
            my_lib.save(path)?;
            println!("Book added!");
        }
        // Commands::Remove {
        //     title,
        //     author,
        //     isbn,
        // } => {
        // my_lib.remove(title.as_deref(), author.as_deref(), isbn.as_deref())?;
        // my_lib.save(path)?;
        // println!("Book removed from your library.");
        // }
        Commands::Remove(rm_args) => {
            my_lib.remove(
                rm_args.title.as_deref(),
                rm_args.author.as_deref(),
                rm_args.isbn.as_deref(),
            )?;
            my_lib.save(path)?;
            println!("Book removed from your library.");
        }
    }

    Ok(())
}
