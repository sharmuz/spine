# spine's Roadmap

Or, more like a list of issues and features I may address at some point :)

## v0.1

- User can `add`, `show` books in their library
- Books can also have a status (mutually exclusive): want, reading, read
- Books can have: title, author, isbn (optional) uid
- Library is persisted locally in a `spine.json` file (in cwd)
- Basic CLI


## v0.2

- User can `remove` books from their library
- Books can have status updated, e.g. want --> reading
- Book details can be edited/added (title, author, isbn)
- Removal/editing/updating are done via book uid
- Library file can be at any user-provided path, with several defaults checked

## v0.3
- User can `tag` books with custom tags, multiple per book possible
- Searching using `show` with author, title, isbn and/or custom tags

## Beyond
- An API service (which?) can be used to `validate` books - adding/correcting data
- Utilise Hardcover.app API (TBD)
- More robust metadata handling via custom types for title/author/isbn
