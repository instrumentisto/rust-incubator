Step 2.8: Logging
=================

[Rust] flexible type system and [metaprogramming][1] capabilities allow to build both efficient and highly reusable log system. The idea is similar to [serde] and in its basic concept is introduced in a widely used [log] and [slog] crates of [Rust] ecosystem.




## Simple logging

Crate [log] represents a single unified frontend interface (facade) which is used by all libraries at the same time, but is backed by one actual backend implementation on your choice. This allows to control all the logs (of application and its dependencies) from a single place and in an unified manner: opt-in and opt-out logs of libraries, separate logs by destinations, etc.

>>>
- Libraries should link only to the log crate, and use the provided macros to log whatever information will be useful to downstream consumers.
- Executables should choose a logger implementation and initialize it early in the runtime of the program. Logger implementations will typically include a function to do this.
>>>

One interesting part is that log levels can be [disabled at compile-time][3], thus have __no runtime performance impact at all__, unless you're debugging.

Read through [log] crate [official documentation][2] for more information and details.




## Structured logging

For [structured logging][4] there is an excellent [slog] crate in [Rust] ecosystem.

>>>
The ambition is to be The Logging Library for Rust. slog should accommodate a variety of logging features and requirements. If there is a feature that you need and standard log crate is missing, slog should have it.
>>>

It's backward and forward compatible with [log] crate, extends its ideas and is baked with an [excellent performance][5].

Read through its [official documentation][6] for better understanding differences with [log] crate.




## Task

Initialize two loggers:
1. Global main `app.log` logger which prints all its logs to `STDOUT`, but `WARN` level (and higher) logs to `STDERR`.
2. Local `access.log` logger which writes all its logs to `access.log` file.

All logs should be structured and logged in a JSON format, and have time field with nanoseconds ([RFC3339] formatted).

Examples:
```json
{"lvl":"ERROR","file":"app.log","time":"2018-07-30T12:14:14.196483657Z","msg":"Error occurred"}
{"lvl":"INFO","file":"access.log","time":"2018-07-30T12:17:18.721127239Z","msg":"http","method":"POST","path":"/some"}
```





[Rust]: https://www.rust-lang.org
[log]: https://crates.io/crates/log
[serde]: https://crates.io/crates/serde
[slog]: https://crates.io/crates/slog
[RFC3339]: https://www.ietf.org/rfc/rfc3339.txt

[1]: https://en.wikipedia.org/wiki/Metaprogramming
[2]: https://docs.rs/log
[3]: https://docs.rs/log/#compile-time-filters
[4]: https://dzone.com/articles/what-is-structured-logging
[5]: https://github.com/slog-rs/slog/wiki/What-makes-slog-fast
[6]: https://docs.rs/slog
