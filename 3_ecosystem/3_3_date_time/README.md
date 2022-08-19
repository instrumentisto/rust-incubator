Step 3.3: Date and time
=======================

__Estimated time__: 1 day

[Rust] has a simple [`std::time`] module which contains very basic primitives for time measurements. To operate with dates, time zones, epochs, and other related stuff, the [`time`] and [`chrono`] crates are used in [Rust] ecosystem.

The main difference between them (except the API, ergonomics and maintaining activity) is that [`chrono`] crate parametrizes time zone in types, while [`time`] crate handles it in runtime. In practice, we recommend to use [`time`] crate (unless [`chrono`] better suits your needs), as it's much actively maintained and evolved.

For better understanding and familiarity, read through the following documentation:
- [Official `std::time` docs][`std::time`]
- [Official `time` crate docs][`time`]
- [Official `chrono` crate docs][`chrono`]




## Duration measurements for code

Beware, that to measure duration of some operation, you should not use [`time`] crate primitives or an [`std::time::SystemTime`], but only an [`std::time::Instant`] instead, as it provides [monotonic clock][1] measurement (otherwise, your time measurement may be inconsistent due to [system clock drift][2]).




## Task

Provide implementations for `User::age()` and `User::is_adult()` methods in [this step's crate](src/main.rs).

Prove your implementation correctness with additional tests. For tests reproducibility consider that "now time" is the date specified in the `NOW` constant.




[`chrono`]: https://docs.rs/chrono
[`std::time`]: https://doc.rust-lang.org/std/time/index.html
[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`time`]: https://docs.rs/time
[Rust]: https://www.rust-lang.org

[1]: https://stackoverflow.com/questions/3523442/difference-between-clock-realtime-and-clock-monotonic
[2]: https://en.wikipedia.org/wiki/Clock_drift
