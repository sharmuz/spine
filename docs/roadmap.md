# spine's Roadmap

Or, more like a list of issues and features I may address at some point :)

## v0.3
- Books each have a uuid, auto-generated on creation
- Remove/update/get_index/tag take a uuid
- CLI (or calling code) handles multiple hits from search (which is unchanged)
- User can `tag` books with custom tags, multiple per book possible
- Searching using `show` with author, title, isbn and/or custom tags
- Library file can be at any user-provided path, with several defaults checked

## Beyond
- More metadata: date read, publisher, translator, edition, comment etc.
- Book details can be added/edited (title, author, isbn)
- More robust metadata handling via custom types for title/author/isbn
- Implement custom Error(s)
- Import/export from/to format compatible with Hardcover/Goodreads/Storygraph
- Leverage an API service (which?) to `validate` books - adding/correcting data
- Utilise Hardcover.app API in other ways (TBD)
- TUI using ratatui
