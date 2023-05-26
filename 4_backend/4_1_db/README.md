Step 4.1: Databases, connection pools and ORMs
==============================================

__Estimated time__: 1 day




## Databases integration

The current situation with databases integration in [Rust] ecosystem is illustrated in [this Awesome Rust section][1]:
some of the drivers represent a native [Rust] implementation, others are wrappers over existing libs, most of them are synchronous, but some provide async interface.


### Connection pool

The important concept to understand is a [connection pool][1] pattern. It's widely adopted in situations where a program represents a long-running application (like daemons or servers). The key point is that __instead of creating a new connection to database every time__ we need to interact with it, we __rather pre-create a pool of such connections and reuse them__. As connection creation is quite an expensive operation, applying this pattern leads to huge performance improvements.

Fortunately, [Rust] ecosystem provides generic implementations of database-agnostic [connection pool][1] in both flavours: sync and async.

For _sync connections_ there is [r2d2] crate. You can easily adopt it for your specific case (or database) just by implementing a required trait. Obviously, there are [implementations for common libraries][3] already.

For _async connections_, unfortunately, there is no such mature implementation at the moment. However, the most used are [bb8] and [l337] crates.

For better understanding [connection pooling][1], read through the following articles:
- [Charlie Custer: What is Connection Pooling, and Why Should You Care][8]


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





[bb8]: https://docs.rs/bb8
[diesel]: https://docs.rs/diesel
[diesel_cli]: https://docs.rs/diesel_cli
[l337]: https://github.com/OneSignal/L3-37
[r2d2]: https://docs.rs/r2d2
[Rust]: https://www.rust-lang.org
[SQLite]: https://www.sqlite.org

[1]: https://github.com/rust-unofficial/awesome-rust#database-1
[2]: https://en.wikipedia.org/wiki/Connection_pool
[3]: https://crates.io/search?q=r2d2
[4]: https://en.wikipedia.org/wiki/Object-relational_mapping
[5]: http://diesel.rs/guides
[6]: https://en.wikipedia.org/wiki/Schema_migration
[7]: https://diesel.rs/guides/getting-started
[8]: https://www.cockroachlabs.com/blog/what-is-connection-pooling
