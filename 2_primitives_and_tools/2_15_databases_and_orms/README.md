Step 2.15: Databases and ORMs
=============================

The current [Rust] ecosystem of databases integration is overviewed in this [Awesome Rust section][1]. Some of the drivers represent a native [Rust] implementation, others are wrappers over existing libs. Most of them are synchronous, but some provide async interface.




## Connection pool

The important concept to understand is a [connection pool][1] pattern. It's widely adopted in situations where the program represents a long-running application (like daemons or servers). The key point is to __not create a new connection__ to a database __every time__ we need to interact with it, __but rather to pre-create a pool of such connections__ and reuse them. As connection creation is quite an expensive operation, applying this pattern leads to huge performance improvements.

Fortunately, [Rust] has a generic implementation of [connection pool][1] in a form of [r2d2] crate that is database-agnostic. You can easily adopt it for your specific case just by implementing required trait. Obviously, there are [implementations for common libraries][3] already. Read through its [documentation][4] to understand better how to use it.




## Diesel

Regarding to [ORM pattern][4], the most feature-rich and mature implementation in [Rust] is the [diesel] crate at the moment. Furthermore, it also comes with [diesel_cli] tool for [migrations][5].

Read through its [documentation][6] and [guides][7] to understand better its design and how to use it.




## Task

Using [actix-web] and [diesel] implement a trivial [RESTful API][8] backed by [SQLite] database, which has the following endpoints:
- `GET /articles` returns a list of all existing `Article`s;
- `GET /article/:id` returns `Article` by its ID;
- `POST /article` creates new `Article` and returns its ID;
- `PUT /article/:id` updates existing `Article` by its ID;
- `DELETE /article/:id` deletes existing `Article` by its ID.

The `Article` entity must consist of the following fields (except `id`):
- `title` - the name of the `Article`;
- `body` - the concrete text that represents an `Article`;
- `labels` - the list of `Article` labels.





[Rust]: https://www.rust-lang.org
[SQLite]: https://www.sqlite.org
[actix-web]: https://crates.io/crates/actix-web
[diesel]: https://crates.io/crates/diesel
[diesel_cli]: https://crates.io/crates/diesel_cli
[r2d2]: https://crates.io/crates/r2d2

[1]: https://github.com/rust-unofficial/awesome-rust#database-1
[2]: https://en.wikipedia.org/wiki/Connection_pool
[3]: https://crates.io/search?q=r2d2-
[4]: https://en.wikipedia.org/wiki/Object-relational_mapping
[5]: https://en.wikipedia.org/wiki/Schema_migration
[6]: http://docs.diesel.rs
[7]: http://diesel.rs/guides
[8]: https://en.wikipedia.org/wiki/Representational_state_transfer
