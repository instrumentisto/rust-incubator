Step 3.12: Web frameworks, databases, connection pools and ORMs
===============================================================

__Estimated time__: 1 day




## Web frameworks and libraries

[This Awesome Rust section][21] illustrates the current situation with [web frameworks][22] and libraries in [Rust] ecosystem quite well.


### Client

For performing HTTP requests there are two main crates: [reqwest] and [hyper].

[reqwest] provides a convenient, higher-level HTTP client in both flavours: sync and async.

[hyper] is a general-purpose async HTTP library, which provides client capabilities.

For better understanding and familiarity with [Rust] HTTP clients, read through the following articles:
- [Official `reqwest` crate docs][reqwest]
- [Official `hyper` crate docs][hyper]
- [Official `hyper` crate guides: Client][23]


### Server

Regarding _async_ HTTP server implementation, the most stable and production-ready are [actix-web] and [hyper] crates at the moment.

[actix-web] is a small, pragmatic, and extremely fast [Rust] web framework. It's build on top of [actix] actor abstractions (so provides good integration with sync world) and is [very careful about performance][24]. Its HTTP support is quite feature-rich and production-tested.

[hyper] is a general-purpose async HTTP library, which aims for _fast_ and _correct_ HTTP implementation for [Rust]. As this is a general-purpose library, using it directly for server implementation can feel quite low-level and unergonomic. However, there are numerous [web frameworks][22] built on top of [hyper], which provide friendly interface: [warp], [tower-web], [rocket].

For better understanding and familiarity with [Rust] HTTP servers, read through the following articles:
- [Official `actix-web` crate docs][actix-web]
- [Official `actix` crate guides: Server][26]
- [Official `hyper` crate guides: Server][25]




## Databases integration

The current situation with databases integration in [Rust] ecosystem is illustrated in [this Awesome Rust section][1]:
some of the drivers represent a native [Rust] implementation, others are wrappers over existing libs, most of them are synchronous, but some provide async interface.


### Connection pool

The important concept to understand is a [connection pool][1] pattern. It's widely adopted in situations where a program represents a long-running application (like daemons or servers). The key point is that __instead of creating a new connection to database every time__ we need to interact with it, we __rather pre-create a pool of such connections and reuse them__. As connection creation is quite an expensive operation, applying this pattern leads to huge performance improvements.

Fortunately, [Rust] ecosystem provides generic implementations of database-agnostic [connection pool][1] in both flavours: sync and async.

For _sync connections_ there is [r2d2] crate. You can easily adopt it for your specific case (or database) just by implementing a required trait. Obviously, there are [implementations for common libraries][3] already.

For _async connections_, unfortunately, there is no such mature implementation at the moment. However, the most used are [bb8] and [l337] crates. 


### ORM

Regarding an [ORM pattern][4], the most feature-rich and mature implementation in [Rust] ecosystem is [diesel] crate at the moment. It's quite well designed, however, supports _only synchronous_ operations.

For better understanding [diesel] design, concepts, usage, and features, read through the following articles:
- [Official `diesel` crate docs][diesel]
- [Official `diesel` crate guides][5]


### Migrations

For [database migrations][6] the obvious choice is [diesel_cli] crate, which can be used even if [diesel] is not used at all.  

For being familiar with [diesel_cli] tool for migrations, read through the following articles:
- [Official `diesel` crate guides: Getting Started][7]
- [Official `diesel_cli` crate docs][diesel_cli]




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




[actix]: https://docs.rs/actix
[actix-web]: https://docs.rs/actix-web
[bb8]: https://docs.rs/bb8
[diesel]: https://docs.rs/diesel
[diesel_cli]: https://docs.rs/diesel_cli
[hyper]: https://docs.rs/hyper
[l337]: https://github.com/OneSignal/L3-37
[r2d2]: https://docs.rs/r2d2
[reqwest]: https://docs.rs/reqwest
[rocket]: https://docs.rs/rocket
[Rust]: https://www.rust-lang.org
[SQLite]: https://www.sqlite.org
[tower-web]: https://docs.rs/tower-web
[warp]: https://docs.rs/warp

[1]: https://github.com/rust-unofficial/awesome-rust#database-1
[2]: https://en.wikipedia.org/wiki/Connection_pool
[3]: https://crates.io/search?q=r2d2-
[4]: https://en.wikipedia.org/wiki/Object-relational_mapping
[5]: http://diesel.rs/guides
[6]: https://en.wikipedia.org/wiki/Schema_migration
[7]: http://diesel.rs/guides/getting-started
[21]: https://github.com/rust-unofficial/awesome-rust#web-programming
[22]: https://en.wikipedia.org/wiki/Web_framework
[23]: https://hyper.rs/guides/client/basic
[24]: https://www.techempower.com/benchmarks/#section=data-r16&hw=ph&test=plaintext
[25]: https://hyper.rs/guides/server/hello-world
[26]: https://actix.rs/docs/server
[30]: https://en.wikipedia.org/wiki/Overengineering
