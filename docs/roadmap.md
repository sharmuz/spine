# spine's Roadmap

Or, more like a list of issues and features I may address at some point :)

## v0.6
- ~~Move cli and tui to be mods under main.rs~~
- Improve TUI
  - ~~Refactor to use `TableState`~~
  - ~~Remove `Widget::render` in favour of dedicated `Tui::render` method~~
  - ~~Refactor `Popup`~~:
    - ~~Rename existing Popup to FilterPopup~~
    - ~~Create Popup ~~trait~~ enum~~
    - ~~Move to separate module popup.rs~~
  - Create ControlsPopup to show keyboard shortcuts
  - Remove magic numbers:
    - Popup area and cursor position
    - `Table` widths
    - Colours and style into `TableTheme` struct
  - Add scrollbar

## Beyond
### General enhancements
- Improve Author representation wrt middle/surnames
- Capitalisation of author and title
- Library file can be at any user-provided path, with defaults at ~/.config/spine and .
- Metadata can be edited (title, author, isbn, tags)
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
