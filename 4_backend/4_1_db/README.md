Step 4.1: Databases, connection pools and ORMs
==============================================

__Estimated time__: 1 day

The current situation with databases integration in [Rust] ecosystem is illustrated quite well in [this "Awesome Rust" section][1] and in ["Database" topic of "Are we web yet?"][2]: the majority of the drivers are implemented fully in [Rust], and only few wrap existing libraries, and of course, most of them use [async I/O][3].




## Connection pool

The important concept to understand is a [connection pool][11] pattern. It's widely adopted in situations where a program represents a long-running application (like [daemons][12] or [servers][13]). The key point is that __instead of creating a new connection to database every time__ we need to interact with, we'd __rather pre-create a [pool][14] of such connections and reuse them__. As connection creation is quite an expensive operation, applying this pattern leads to huge performance improvements.

Fortunately, [Rust] ecosystem provides generic implementations of database-agnostic [connection pool][1] in both flavours: synchronous and asynchronous.

For better understanding [connection pooling][1], read through the following articles:
- [Charlie Custer: What is Connection Pooling, and Why Should You Care][15]


### Synchronous

For synchronous connections there is the [`r2d2`] crate (the pioneer among such crates, existed far before [async I/O][3] has landed in [Rust]). You can easily adopt it for your specific use-case (or database) just by implementing [its traits][22]. Obviously, there are [implementations for common drivers][21] already.

For more details, read through the following articles:
- [Official `r2d2` crate docs][`r2d2`]


### Asynchronous

For asynchronous connections there are much more options in [Rust] ecosystem, due to historical reasons and bigger competitiveness (as the result of bigger [async I/O][3] popularity).

The very first one, historically, was the [`bb8`] crate. It mirrors the [`r2d2`] crate for asynchronous connections ([`tokio`] only), and originally was based on it. Similarly, there are [implemented bridges for common drivers][23] already.

[`deadpool`] is an [alternative and very mature][25] implementation of the [connection pool][11] pattern, supporting both [`tokio`] and [`async-std`], provided with [its own large ecosystem][24].

Another alternative implementation is the [`mobc`] crate, yet inspired by [`deadpool`] and [`r2d2`] crates. Similarly, supports both [`tokio`] and [`async-std`] and provides some [bridges for common drivers][26].

[`qp`] (Quick Pool) is a very simple and [limited][29] implementation of the [connection pool][11] pattern, [utilizing lock-free primitives][27] and [focused on being performant][28].

For more details, read through the following articles:
- [Official `bb8` crate docs][`bb8`]
- [Official `deadpool` crate docs][`deadpool`]
- [Official `mobc` crate docs][`mobc`]
- [Official `qp` crate docs][`qp`]




## ORM

Regarding the [ORM pattern][41], there are [multiple][42] feature-rich and mature implementation in [Rust] ecosystem at the moment. Every one has its own unique design, advantages and disadvantages.

The very first [ORM][41] created in [Rust] was the [`diesel`] crate. Even now, it supports [only synchronous][43] connections (as was created before [async I/O][3] has landed in [Rust]). However, still may be used with asynchronous connections, thankfully to the [`diesel-async`] extension.

[`sea-orm`] (built on top of [`sea-query`]) is an alternative feature-rich and [mature][46] implementation of the [ORM] pattern in [Rust], focused on [dynamic querying to avoid complexity of static checks ("fighting the ORM")][47].

[`ormx`] is a lightweight extension of the [`sqlx`] crate, aimed to provide it with [ORM][41]-like features.

[`rustorm`] is a very simple and [SQL]-centered [ORM][41], focused on easing conversions of database types to their appropriate [Rust] types.

For better understanding [ORMs][41] design, concepts, usage, and features, read through the following articles:
- [Official `diesel` crate docs][`diesel`]
- [Official `diesel` crate guides][44]
- [Official `sea-orm` crate docs][`sea-orm`]
- [Official `sea-orm` crate guides][45]
- [Official `ormx` crate docs][`ormx`]
- [Official `rustorm` crate docs][`rustorm`]




### Migrations

For [database migrations][61] there are [multiple tools][62] in [Rust] ecosystem.

For [`diesel`] users, the obvious choice is the [`diesel_migrations`] crate (which may be used directly via [`diesel_cli`]). Though, doesn't require the [`diesel`] itself to be used, and may be used as a fully separate tool.

For [`sqlx`] users, similarly, the [`sqlx-cli`] tool [provides migrations][64] out-of-the-box, while also may be used [directly in the application code][65].

[`refinery`] and [`migrant`] are another standalone [Rust] tools for [migrations][61], allowing both [CLI] and ["in-application-code"][66] usage. The interesting part about the [`refinery`] crate is that it also allows to write "in-application-code" [migrations][61] via the [`barrel`] schema migration builder.

For being familiar with [migrations][61] tools, their similarities and differences, read through the following articles:
- [Official `diesel_migrations` crate docs][`diesel_migrations`]
- [Official `diesel_cli` crate docs][`diesel_cli`]
- [Official `diesel` crate guides: Getting Started][63]
- [Official `sqlx` crate docs: Macro `sqlx::migrate`][65]
- [Official `refinery` crate docs][`refinery`]
- [Official `migrant` crate docs][`migrant`]




[`async-std`]: https://docs.rs/async-std
[`barrel`]: https://docs.rs/barrel
[`bb8`]: https://docs.rs/bb8
[`deadpool`]: https://docs.rs/deadpool
[`diesel`]: https://docs.rs/diesel
[`diesel_cli`]: https://docs.rs/diesel_cli
[`diesel_migrations`]: https://docs.rs/diesel_migrations
[`diesel-async`]: https://docs.rs/diesel-async
[`migrant`]: https://docs.rs/migrant
[`mobc`]: https://docs.rs/mobc
[`ormx`]: https://docs.rs/ormx
[`qp`]: https://github.com/Astro36/qp
[`r2d2`]: https://docs.rs/r2d2
[`refinery`]: https://docs.rs/refinery
[`rustorm`]: https://github.com/ivanceras/rustorm
[`sea-orm`]: https://docs.rs/sea-orm
[`sea-query`]: https://docs.rs/sea-query
[`sqlx`]: https://docs.rs/sqlx
[`sqlx-cli`]: https://crates.io/crates/sqlx-cli
[`tokio`]: https://docs.rs/tokio
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[Rust]: https://www.rust-lang.org
[SQL]: https://en.wikipedia.org/wiki/SQL

[1]: https://github.com/rust-unofficial/awesome-rust#database-1
[2]: https://www.arewewebyet.org/topics/database
[3]: ../../3_ecosystem/3_11_async
[11]: https://en.wikipedia.org/wiki/Connection_pool
[12]: https://en.wikipedia.org/wiki/Daemon_(computing)
[13]: https://en.wikipedia.org/wiki/Server_(computing)
[14]: https://en.wikipedia.org/wiki/Object_pool_pattern
[15]: https://www.cockroachlabs.com/blog/what-is-connection-pooling
[21]: https://crates.io/search?q=r2d2
[22]: https://docs.rs/r2d2#traits
[23]: https://crates.io/search?q=bb8
[24]: https://crates.io/search?q=deadpool
[25]: https://docs.rs/deadpool#reasons-for-yet-another-connection-pool
[26]: https://crates.io/search?q=mobc
[27]: https://github.com/Astro36/qp#bb8-vs-qp
[28]: https://github.com/Astro36/qp#performance-comparison
[29]: https://github.com/Astro36/qp#dbcp
[41]: https://en.wikipedia.org/wiki/Object-relational_mapping
[42]: https://www.arewewebyet.org/topics/database#orms
[43]: https://github.com/diesel-rs/diesel/issues/399
[44]: https://diesel.rs/guides
[45]: https://www.sea-ql.org/SeaORM/docs/index
[46]: https://docs.rs/sea-orm#whos-using-seaorm
[47]: https://www.sea-ql.org/SeaORM/docs/latest/internal-design/diesel#programming-paradigm
[61]: https://en.wikipedia.org/wiki/Schema_migration
[62]: https://www.arewewebyet.org/topics/database#tooling
[63]: https://diesel.rs/guides/getting-started
[64]: https://crates.io/crates/sqlx-cli#create-and-run-migrations
[65]: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
[66]: https://docs.rs/refinery/latest/refinery/macro.embed_migrations.html
