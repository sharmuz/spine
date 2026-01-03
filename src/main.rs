use std::{io, path::Path, str::FromStr};

use clap::{Args, CommandFactory, Parser, Subcommand};
use uuid::Uuid;

use spine::{Book, Library, LibrarySearch, Status};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show your books
    Show(ShowArgs),

    /// Add a new book
    Add(AddArgs),

    /// Remove an existing book
    Remove(SearchArgs),

    /// Update an existing book
    #[command(subcommand)]
    Update(UpdateType),
}

#[derive(Args)]
struct AddArgs {
    title: String,

    author: String,

    #[arg(short, long)]
    isbn: Option<String>,

    #[command(flatten)]
    status: StatusFlag,

    #[arg(long, alias = "tag", value_delimiter = ',')]
    tags: Vec<String>,
}

#[derive(Args)]
struct ShowArgs {
    #[arg(long)]
    all: bool,

    #[command(flatten)]
    search: SearchArgs,
}

#[derive(Args)]
#[group(required = false, multiple = true)]
struct SearchArgs {
    #[arg(short, long)]
    title: Option<String>,

    #[arg(short, long)]
    author: Option<String>,

    #[arg(short, long)]
    isbn: Option<String>,

    #[arg(short, long)]
    status: Option<String>,

    #[arg(long, alias = "tag", value_delimiter = ',')]
    tags: Option<Vec<String>>,
}

impl SearchArgs {
    fn is_any_set(&self) -> bool {
        self.title.is_some()
            || self.author.is_some()
            || self.isbn.is_some()
            || self.status.is_some()
            || self.tags.is_some()
    }
}

#[derive(Subcommand)]
enum UpdateType {
    /// Update the status of an existing book
    Status {
        #[command(flatten)]
        status: StatusFlag,

        #[command(flatten)]
        search: SearchArgs,
    },
}

#[derive(Args)]
#[group(required = false, multiple = false)]
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let path = Path::new("spine.json");
    let mut my_lib = if path.exists() {
        Library::open(path)?
    } else {
        Library::new()
    };

    match cli.command {
        Commands::Show(show_args) => {
            if show_args.all && show_args.search.is_any_set() {
                let mut cmd = Cli::command();
                let msg = concat!("--all is mutually exclusive with search criteria.");
                cmd.error(clap::error::ErrorKind::ArgumentConflict, msg)
                    .exit();
            }

            println!("Books in your library:\n");
            if show_args.search.is_any_set() {
                let hits = get_search_hits(&my_lib, &show_args.search)?;
                hits.iter().for_each(|b| println!("{b}"));
            } else {
                my_lib.all().for_each(|b| println!("{b}"));
            }
        }
        Commands::Add(add_args) => {
            let my_book = Book {
                title: add_args.title,
                author: add_args.author,
                isbn: add_args.isbn,
                status: add_args.status.to_status(),
                tags: add_args.tags,
                ..Default::default()
            };
            my_lib.add(my_book);
            my_lib.save(path)?;
            println!("Book added!");
        }
        Commands::Remove(search_args) => {
            if !search_args.is_any_set() {
                let mut cmd = Cli::command();
                let msg = concat!("no search criteria provided.");
                cmd.error(clap::error::ErrorKind::MissingRequiredArgument, msg)
                    .exit();
            }

            let hits = get_search_hits(&my_lib, &search_args)?;
            let rm_id = select_books(hits)?;
            my_lib.remove(rm_id)?;
            my_lib.save(path)?;
            println!("Book removed from your library.");
        }
        Commands::Update(update_type) => match update_type {
            UpdateType::Status { status, search } => {
                if !status.is_set() {
                    let mut cmd = Cli::command();
                    let msg = concat!(
                        "the following required arguments were not provided:\n",
                        "  <--want|--reading|--read>."
                    );
                    cmd.error(clap::error::ErrorKind::MissingRequiredArgument, msg)
                        .exit();
                }
                if !search.is_any_set() {
                    let mut cmd = Cli::command();
                    let msg = concat!("no search criteria provided.");
                    cmd.error(clap::error::ErrorKind::MissingRequiredArgument, msg)
                        .exit();
                }

                let hits = get_search_hits(&my_lib, &search)?;
                let update_id = select_books(hits)?;
                let new_status = status.to_status();
                my_lib.update_status(update_id, new_status)?;
                my_lib.save(path)?;
                println!("Book status updated to {new_status:?}.");
            }
        },
    }

    Ok(())
}

fn get_search_hits<'a>(lib: &'a Library, search: &SearchArgs) -> Result<Vec<&'a Book>, io::Error> {
    Ok(lib.search(&LibrarySearch {
        title: search.title.as_deref(),
        author: search.author.as_deref(),
        isbn: search.isbn.as_deref(),
        status: search
            .status
            .as_deref()
            .map(Status::from_str)
            .transpose()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?,
        tags: search.tags.as_ref(),
        ..Default::default()
    }))
}

fn select_books(hits: Vec<&Book>) -> Result<Uuid, io::Error> {
    if hits.is_empty() {
        return Err(io::Error::other("No books found matching given criteria."));
    } else if hits.len() > 1 {
        return Err(io::Error::other(
            "Please be more specific, found multiple books.",
        ));
    }

    Ok(hits[0].id)
}
