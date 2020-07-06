Step 3.11: Async I/O, futures and actors
========================================

__Estimated time__: 2 days

While [threads](../3_10_threads) represent a solution for [CPU-bound] problems, for [I/O-bound] problems, traditionally, the solution is [async I/O][1].

[Rust] has no async primitives in its standard library yet, so "by default" `std` I/O works in a synchronous manner (blocks the current thread). However, there is a strong foundation for async programming with [futures] and [tokio] crates, which cover the core abstractions for async and primitives for [async I/O][1].

It's important to note, that async story in [Rust] is still maturing and on its [way to stabilization][2]. That's why things are quite cumbersome at the moment. This should become easier, simpler, more ergonomic and powerful in a near future.




## Futures

The basic primitive of async story in [Rust] is a [future abstraction][3] (may be called "promise" in other programming languages). There are to major concepts which differ [Rust implementation of futures][4] from other programming languages:
1. Futures are [poll-based][5] rather than push-based. This means that after creation, a future is not going to be executed automatically in-place, but rather should be explicitly executed by some executor (runtime/event-loop for futures). __Future does nothing unless polled__, so generally represents a [lazy computation][7].
2. Futures are [zero cost][7]. This means that the code written on futures compiles down to something equivalent (or better than) a “hand-rolled” implementation, which would typically use manual state machines and careful memory management.

To understand [Rust] futures concepts and design better, read through the following articles:
- [Aaron Turon: Zero-cost futures in Rust][6]
- [Aaron Turon: Designing futures for Rust][4]
- [Rust RFC 2592: `futures_api`][8]

[Rust] provides only basic trait definitions in its [`std::future`] module of standard library. To use futures with all its power, you should use [futures] crate.

Important to know, that before [futures design has been stabilized][8], for a quite long period of time [Rust] ecosystem used [futures 0.1] crate, which resulted in a big part of ecosystem being built on top of them. Fortunately, current [futures] and [futures 0.1] can be used simultaneously due to [compatibility layer][9].

For better understanding [futures] usage, and features, read through the following articles:
- [Diggory Blake: Looking into the `Future`][10] (describes [futures 0.1])
- [Francesco Cogno: Rust futures: an uneducated, short and hopefully not boring tutorial][11] (describes [futures 0.1])
- [Michael Snoyman: Async, futures, and tokio - Rust Crash Course lesson 7][12] (describes [futures 0.1])
- [Official `futures` crate docs][futures]

Additional information my be taken from a slightly outdated [Async in Rust] book.




## Async I/O

The async programming is not possible without a support for [non-blocking I/O][1].

[Rust] ecosystem has [mio] crate, which provides very _raw primitives_ for [async I/O][1], and [tokio] crate, which provides _high-level [futures]-based abstractions_ for using [async I/O][1].

For better understanding [mio] and [tokio] design, concepts, usage, and features, read through the following articles:
- [Official `mio` crate docs][mio]
- [Official `tokio` crate docs][tokio]
- [Official `tokio` crate guide][13]
- [Nick Cameron: Asynchronous programming with Rust: Introduction][14]




## Async/await and runtime

The next step for async story in [Rust] is stabilization of [`async` & `await` syntaxes][20], which makes async programming much more intuitive, ergonomic, and solves numerous problems with types and borrows (which may quite tricky when using raw [futures]).

[Rust] ecosystem is also preparing an excellent [runtime] crate, which is aimed on introducing an ergonomic and transparent abstraction over a [futures] runtime.

For better understanding `async`/`await` design, concepts, usage, and features, read through the following articles:
- [Rust RFC 2394: `async_await`][20]
- [Nick Cameron: Asynchronous programming with Rust: `async` and `await`][21]
- [Bastian Gruber: Explained: How does async work in Rust?][22]
- [Official `runtime` crate docs][runtime]




## Actors

[Actor model][41] is another very spread and famous concurrency programming paradigm. It fits quite good for solving major concurrent communication problems, so many languages adopted it as their main [concurrency paradigm][2] (the most famous implementation is [Akka]).

[Rust] has a well designed and performant [actor model][41] implementation in a form of [actix] crate. Also, [actix] provides both sync actors and async actors implementations, which makes this crate very good as __a "glue" to unite sync/async worlds__, which is a major problem of nowadays [Rust] (because many libraries still act only in a synchronous manner).

For better understanding [actix] actors design, concepts, usage, and features, read through the following articles:
- [Official `actix` crate docs][actix]
- [Official `actix` crate guide][43]




## Task

Implement an async-driven CLI tool, which downloads specified web pages:
```bash
cargo run -p step_3_11 -- [--max-threads=<number>] <file>
```
It must read a list of links from the `<file>`, and then concurrently download a content of each link into a separate `.html` file (named by a link).

`--max-threads` argument must control the maximum number of simultaneously running threads in the program (should default to CPUs number).

It should compile on stable [Rust].





[actix]: https://docs.rs/actix
[Akka]: https://akka.io
[Async in Rust]: https://rust-lang.github.io/async-book/index.html
[CPU-bound]: https://en.wikipedia.org/wiki/CPU-bound
[futures]: https://docs.rs/futures-preview
[futures 0.1]: https://docs.rs/futures/0.1
[futures 0.2]: https://docs.rs/futures/0.2
[I/O-bound]: https://en.wikipedia.org/wiki/I/O_bound
[mio]: https://docs.rs/mio
[runtime]: https://docs.rs/runtime
[Rust]: https://www.rust-lang.org
[`std::future`]: https://doc.rust-lang.org/std/future/index.html
[tokio]: https://docs.rs/tokio

[1]: https://en.wikipedia.org/wiki/Asynchronous_I/O
[2]: https://areweasyncyet.rs
[3]: https://en.wikipedia.org/wiki/Futures_and_promises
[4]: https://aturon.github.io/blog/2016/09/07/futures-design
[5]: http://aturon.github.io/blog/2016/09/07/futures-design/#what-worked-the-demand-driven-aka-readiness-based-approach
[6]: https://aturon.github.io/blog/2016/08/11/futures
[7]: https://en.wikipedia.org/wiki/Lazy_evaluation
[8]: https://rust-lang.github.io/rfcs/2592-futures.html
[9]: https://rust-lang-nursery.github.io/futures-rs/blog/2019/04/18/compatibility-layer.html
[10]: https://gist.github.com/Diggsey/6f924bf3f741bcdffd240faee102fe92
[11]: https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part-1-3k3
[12]: https://www.snoyman.com/blog/2018/12/rust-crash-course-07-async-futures-tokio
[13]: https://tokio.rs/docs/overview
[14]: https://github.com/nrc/apr-intro/blob/master/intro.md
[20]: https://rust-lang.github.io/rfcs/2394-async_await.html
[21]: https://github.com/nrc/apr-intro/blob/master/async-await.md
[22]: https://dev.to/gruberb/explained-how-does-async-work-in-rust-46f8
[41]: https://en.wikipedia.org/wiki/Actor_model
[42]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[43]: https://actix.rs/book/actix
