# spine's Roadmap

Or, more like a list of issues and features I may address at some point :)

## ~~v0.5~~
- ~~TUI using ratatui:~~
  - ~~Move current main.rs to cli.rs~~
  - ~~Create new main.rs which loads cli or tui~~
  - ~~Create tui.rs told hold Tui struct with appropriate methods (ELM arch?)~~
  - ~~Display all books in library with scrollable view and movable cursor~~
  - ~~Filter books by status via key shortcut~~
  - ~~Filter books by tags via user input~~
  - ~~Prettify book list:~~
    - ~~show status~~
    - ~~highlight current row with bold/bg colour~~
    - ~~column width~~
    - ~~column headers~~
    - ~~show tags~~
    - ~~spacing between rows and cols~~

## v0.6
- Improve TUI
  - Refactor to use `TableState`
  - Add scrollbar
  - Refactor Popup
    - Rename existing Popup to FilterPopup
      - Remove magic number for cursor position
    - Create Popup trait?
    - Create ControlsPopup to show keyboard shortcuts
    - Move to separate module popup.rs

## Beyond
### General enhancements
- Improve Author representation wrt middle/surnames
- Capitalisation of author and title
- Library file can be at any user-provided path, with defaults at ~/.config/spine and .
- Metadata can be added/edited (title, author, isbn, tags)
- More metadata: year, date read, publisher, translator, edition, comment etc.
- More integration tests
- Implement custom Error(s)
- Import/export from/to format compatible with Hardcover/Goodreads/Storygraph
- Leverage an API service (OpenLibrary?) to `validate` books - adding/correcting data

### Additional features for TUI
- Add new books
- Sort books
- Inspect individual books
- Remove books
- Update/edit books
- Save filters/searches
