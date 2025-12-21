# spine's Roadmap

Or, more like a list of issues and features I may address at some point :)

## v0.2

- Restructure crate
- Books each have a uid
- User can `remove` books from their library
- Books can have status updated, e.g. want --> reading
- Book details can be edited/added (title, author, isbn)
- Removal/editing/updating are done via book uid

## v0.3
- Library file can be at any user-provided path, with several defaults checked
- User can `tag` books with custom tags, multiple per book possible
- Searching using `show` with author, title, isbn and/or custom tags

## Beyond
- More metadata: date read, publisher, translator, edition, etc.
- More robust metadata handling via custom types for title/author/isbn
- Import/export from/to format compatible with Hardcover/Goodreads/Storygraph
- Leverage an API service (which?) to `validate` books - adding/correcting data
- Utilise Hardcover.app API in other ways (TBD)
- TUI using ratatui
