Step 3.3: Date and time
=======================

__Estimated time__: 1 day

[Rust] has a simple [`std::time`] module which contains very basic primitives for time measurements. To operate with dates, time zones, epochs, and other related stuff, the [chrono] crate is used in [Rust] ecosystem.

For better understanding and familiarity, read through the following documentation:
- [Official `std::time` docs][`std::time`]
- [Official `chrono` crate docs][chrono]




## Duration measurements for code

Beware, that to measure duration of some operation, you should not use [chrono] crate primitives or a [`std::time::SystemTime`], but only a [`std::time::Instant`] instead, as it provides [monotonic clock][1] measurement (otherwise, your time measurement may be inconsistent).




## Task

Provide implementations for `User::age()` and `User::is_adult()` methods in [this step's crate](src/main.rs).

Prove your implementation correctness with additional tests. For tests reproducibility consider that "now time" is the date specified in the `NOW` constant.





[chrono]: https://docs.rs/chrono
[Rust]: https://www.rust-lang.org
[`std::time`]: https://doc.rust-lang.org/std/time/index.html
[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html

[1]: https://stackoverflow.com/questions/3523442/difference-between-clock-realtime-and-clock-monotonic
