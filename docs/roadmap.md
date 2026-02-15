# spine's Roadmap

Or, more like a list of issues and features I may address at some point :)

## v0.5
- TUI using ratatui:
  - ~~Move current main.rs to cli.rs~~
  - ~~Create new main.rs which loads cli or tui~~
  - ~~Create tui.rs told hold Tui struct with appropriate methods (ELM arch?)~~
  - ~~Display all books in library with scrollable view and movable cursor~~
  - Filter books by search criteria
  - Prettify book list: show status/tags, bigger font, spacing, capitalisation

## Beyond
- More integration tests
- Metadata can be added/edited (title, author, isbn, tags)
- More metadata: year, date read, publisher, translator, edition, comment etc.
- Implement custom Error(s)
- Library file can be at any user-provided path, with several defaults checked
- Import/export from/to format compatible with Hardcover/Goodreads/Storygraph
- Leverage an API service (which?) to `validate` books - adding/correcting data
- Utilise Hardcover.app API in other ways (TBD)
- Improve TUI:
  - Add new books
  - Sort books
  - Inspect individual books
  - Update/edit books
  - Remove books
  - Save filters/searches
