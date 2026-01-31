# spine's Roadmap

Or, more like a list of issues and features I may address at some point :)

## v0.5
- TUI using ratatui:
  - Move current main.rs to cli.rs
  - Create new main.rs which loads cli or tui
  - Create tui.rs told hold Tui struct with methods:
    - fn new: likely just Self::default()
    - fn run: core loop to run the tui which calls terminal.draw then self.handle_event
    - fn handle_event: match on key or mouse, calling self.handle_key_event as necessary
    - fn handle_key_event: match on specific keys
    - fn render: draw the tui, called by terminal.draw in run

## Beyond
- More integration tests
- Metadata can be added/edited (title, author, isbn, tags)
- More metadata: year, date read, publisher, translator, edition, comment etc.
- Implement custom Error(s)
- Library file can be at any user-provided path, with several defaults checked
- Import/export from/to format compatible with Hardcover/Goodreads/Storygraph
- Leverage an API service (which?) to `validate` books - adding/correcting data
- Utilise Hardcover.app API in other ways (TBD)
