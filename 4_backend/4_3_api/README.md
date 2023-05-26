Step 4.3: API servers, clients and tools
========================================

__Estimated time__: 1 day





## Task

Using [actix-web] implement a trivial API, backed by [SQLite] database, which has the following endpoints:
- `GET /articles` returns a list of all existing `Article`s;
- `GET /article/:id` returns `Article` by its ID;
- `POST /article` creates new `Article` and returns its ID;
- `DELETE /article/:id` deletes existing `Article` by its ID.

The `Article` entity must consist of the following fields (except `id`):
- `title` - the name of the `Article`;
- `body` - the concrete text that represents an `Article`;
- `labels` - the list of `Article` labels.

Avoid architecture [overengineering][30] for this task, just use simple and obvious solutions.





[actix-web]: https://docs.rs/actix-web
[SQLite]: https://www.sqlite.org

[30]: https://en.wikipedia.org/wiki/Overengineering
