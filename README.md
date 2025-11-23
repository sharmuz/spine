# spine

spine is a simple CLI tool for managing your book collection. It can store your data locally as well as sync with your account on popular websites [Hardcover](https://hardcover.app) or [Storygraph](https://thestorygraph.com/).

## Quickstart

```shell
# Add a new book you've read!
spine add --read "the great gatsby"

# Add another two you want to read
spine add --want "blood meridian"

# See your all books!
spine show --all
```

## Installation

Install via [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```shell
cargo install --locked spine-books
```

## Usage

### Adding a new book

```shell
# Add a new book you've read by title
spine add --read "the great gatsby"

# Add multiple books at once (neither read nor wanted for now!)
spine add "sense and sensibility" "mansfield park" "emma"

# Or add by ISBN!
spine add --read "9781847496140"

# Add a book you want to read
spine add --want "animal farm"
```

### Tag your books

```shell
# Tag an existing book
spine tag "british" "animal farm"

# Tag when you add a new book
spine add --tag "russian" "hadji murat"

# Remove a tag from a book
spine tag --remove "comedy" "white nights"
```

### View your books

```shell
# Show all books you want to read
spine show --want

# Show all books under a tag
spine show --tag "biography"

# Show all books by an author
spine show --author "tolstoy"
```
