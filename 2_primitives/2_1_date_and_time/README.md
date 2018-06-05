Step 2.1: Date and time
=======================

[Rust Standard Library] provides a simple [`time`][`std::time`] module which contains basic primitives for time measurements. To operate with dates, time zones, epochs, and other related stuff, [Rust] has the [Chrono] crate.




## Time measurements

To measure a time of some operation, you should not use [Chrono] crate primitives or [`std::time::SystemTime`], but only [`std::time::Instant`] instead, as it provides [monotonic clock][1] measurement (otherwise, your time measurement may be inconsistent).




## Task

Implement a struct `User` with a `birthdate` field, and its methods:
- `age()` which returns current age of user in years;
- `is_adult()` which checks if user is 18 years old at the moment.   





[Chrono]: https://crates.io/crates/chrono
[Rust]: https://www.rust-lang.org
[Rust Standard Library]: https://doc.rust-lang.org/std
[`std::time`]: https://doc.rust-lang.org/stable/std/time/index.html
[`std::time::Instant`]: https://doc.rust-lang.org/stable/std/time/struct.Instant.html
[`std::time::SystemTime`]: https://doc.rust-lang.org/stable/std/time/struct.SystemTime.html

[1]: https://stackoverflow.com/questions/3523442/difference-between-clock-realtime-and-clock-monotonic
