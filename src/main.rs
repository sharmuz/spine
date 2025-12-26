use std::{error::Error, path::Path};

use clap::{Args, CommandFactory, Parser, Subcommand, error::ErrorKind};

use spine::{Book, Library, LibrarySearch, Status};

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

    /// Update an existing book
    #[command(subcommand)]
    Update(UpdateType),
}

#[derive(Subcommand)]
enum UpdateType {
    /// Update the status of an existing book
    Status {
        #[command(flatten)]
        status: StatusFlag,

        #[command(flatten)]
        search: RemoveArgs,
    },
}

#[derive(Args)]
#[group(multiple = false)]
struct StatusFlag {
    #[arg(long)]
    want: bool,

    #[arg(long)]
    reading: bool,

    #[arg(long)]
    read: bool,
}

impl StatusFlag {
    fn to_status(&self) -> Status {
        match (self.reading, self.read) {
            (true, _) => Status::Reading,
            (_, true) => Status::Read,
            _ => Status::Want,
        }
    }

    fn is_set(&self) -> bool {
        self.want || self.reading || self.read
    }
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
            println!("Books in your library:\n");
            my_lib.all().for_each(|b| println!("{b}"));
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
            let my_book = Book {
                title,
                author,
                isbn,
                status,
            };
            my_lib.add(my_book);
            my_lib.save(path)?;
            println!("Book added!");
        }
        Commands::Remove(rm_args) => {
            my_lib.remove(LibrarySearch {
                title: rm_args.title.as_deref(),
                author: rm_args.author.as_deref(),
                isbn: rm_args.isbn.as_deref(),
            })?;
            my_lib.save(path)?;
            println!("Book removed from your library.");
        }
        Commands::Update(update) => match update {
            UpdateType::Status { status, search } => {
                if !status.is_set() {
                    let mut cmd = Cli::command();
                    let msg = concat!(
                        "the following required arguments were not provided:\n",
                        "  <--want|--reading|--read>."
                    );
                    cmd.error(ErrorKind::MissingRequiredArgument, msg).exit();
                }
                let new_status = status.to_status();
                my_lib.update_status(
                    LibrarySearch {
                        title: search.title.as_deref(),
                        author: search.author.as_deref(),
                        isbn: search.isbn.as_deref(),
                    },
                    new_status,
                )?;
                my_lib.save(path)?;
                println!("Book status updated to {:?}.", new_status);
            }
        },
    }

    Ok(())
}
