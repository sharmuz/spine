# spine

spine is a simple CLI/TUI tool for managing your book collection.

## Quickstart

### CLI

```shell
# Add a new book you've read!
spine add --read "the great gatsby" "f. scott fitzgerald"

# Add one you want to read
spine add --want "burmese days" "george orwell"

# See all your books!
spine show
```

### TUI

```shell
spine --tui
```

## Installation

Install via [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```shell
cargo install --locked spine-books
```

## Usage

### Adding a new book

```shell
# Add a new book you've read by title (by default marked as want)
spine add "sense and sensibility" "jane austen"

# Add a book you want to read
spine add --want "rashomon" "ryunosuke akutagawa"

# Optionally include ISBN
spine add --reading "the great gatsby" "f. scott fitzgerald" --isbn "9781847496140"
```

### View your books

```shell
# Show all books
spine show --all

# Show all books by an author
spine show --author "tolstoy"

# Show all books you want to read
spine show --status want

# Show all books under a tag
spine show --tag "biography"
```

### Tag your books

```shell
# Tag when you add a new book
spine add --tag "russian" "hadji murat" "leo tolstoy"
```

### Update your books

```shell
# Update status of a book
spine update status --read --title "snow crash"
```

### Removing a book

```shell
# Remove a book
spine remove --title "far from the madding crowd"
```
