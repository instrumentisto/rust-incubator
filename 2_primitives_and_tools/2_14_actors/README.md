Step 2.14: Actors
=================

[Actor model][1] is another very spread and famous [concurrency programming][2] paradigm. It fits quite good for solving major concurrent communication problems, so many languages adopted it as their main [concurrency paradigm][2] (the most famous implementation is [Akka]).

[Rust] has a well designed and performant [actor model][1] implementation in a form of [actix] crate. Read through its [documentation][3] and [guide][4] to become familiar with its concepts and usage.

Furthermore, the [actix-web] crate provides [extremely fast][5] and feature-rich client/server HTTP implementation and web framework. As [actix] provides both sync actors and async actors that makes it very good as __a "glue" to unite sync/async worlds__, which is a major problem of nowadays [Rust] (because many libraries act only in synchronous manner). 




## Task

Implement the task from [Step 2.12] by using [actix].

>>>
Implement the following lifecycle:
- `Producer` continuously generates square matrixes of random `u8` elements and size `4096`.
- `Consumer` takes generated matrix, counts sum of all its elements and prints the sum to STDOUT.
- There are only 1 `Producer` and 2 `Consumer`s. Each has its own separate thread.
- Counting sum of matrix elements must be parallelized. 
>>>





[Akka]: https://akka.io
[Rust]: https://www.rust-lang.org
[actix]: https://crates.io/crates/actix
[actix-web]: https://crates.io/crates/actix-web

[1]: https://en.wikipedia.org/wiki/Actor_model
[2]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[3]: https://docs.rs/actix
[4]: https://actix.rs/book/actix
[5]: https://www.techempower.com/benchmarks/#section=data-r16&hw=ph&test=plaintext

[Step 2.12]: ../2_12_threads_synchronization_and_parallelism#task
