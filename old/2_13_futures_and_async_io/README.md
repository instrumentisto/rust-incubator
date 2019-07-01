Step 2.13: Futures and async I/O
================================

[Rust] has no async primitives in its standard library yet, so "by default" everything work in a synchronous manner (blocks the current thread). However, there is a strong foundation for async programming with the [futures] and [tokio] crates, which cover the core abstractions for async, and primitives for [async I/O][1], respectively.

It's important to note, that async story in [Rust] is [still maturing][2] and on its way to [stabilization][3], that's why things are quite cumbersome at the moment. This should become easier, simpler, more ergonomic and powerful in the near future.

The [Async in Rust] book is a comprehensive and up-to-date guide on the async story in [Rust]. This this the main knowledge source you should use. Additional understanding may be obtained from ["Introduction to asynchronous programming in Rust"][18] article.




## Futures

The basic primitive of async story in [Rust] is the [future abstraction][4] (may be called "promise" in other programming languages). There are to major concepts which differ [Rust implementation][5] of futures from the one in other programming languages:
1. Futures are [poll-based][6] rather than push-based. This means that after creation the future is not going to be executed automatically in-place, but it should be explicitly executed by some executor (which represents a some form of runtime for futures).
2. Futures are [zero cost][7]. This means that the code written on futures compiles down to something equivalent (or better than) “hand-rolled” server implementations, which would typically use manual state machines and careful memory management.

To understand the [futures] design better, read the following articles:
- [Zero-cost futures in Rust][7]
- [Designing futures for Rust][6]
- [Async in Rust: 2. Tasks and executors][8]
- [Async in Rust: 5. Futures][9]
- [Tokio: Futures][15]

To understand better how to implement, use and compose [futures], read the following articles:
- [Looking into the `Future`][13]
- [Rust futures: an uneducated, short and hopefully not boring tutorial][14]


### Crate versions

The current situation with futures implementation is quite messed up because of continuous design changes, that's why there are three main versions of [futures] crate at the moment:
- [0.1][10] is the first futures design that was widely adopted by [Rust] ecosystem. This is __the version you should use at the moment__.
- [0.2][11] is the second futures design which has been failed. __Do not use this version at all.__
- [0.3][12] is the latest futures design that is in active development at the moment, but requires [nightly Rust] and has not been adopted by [Rust] ecosystem yet, so is __not usable at the moment__.




## Tokio

The async programming is not possible without having support for [non-blocking I/O][1]. The [tokio] crate in [Rust] is responsible for providing the one. Its components and features are described well in the [official documentation][16].

[tokio] is fully driven by [futures], that's why these two crate very often may be meet in pair.

To understand better the design and usage of [tokio] read though its [guide][17].




## Task

Implement a [futures]-driven CLI tool that downloads web pages:
```bash
./download [--max-threads=<number>] <file>
```

It must read a list of links from the `<file>`, and concurrently download a content of each link into a separate `.html` file (named by a link).

`--max-threads` argument must control the maximum number of simultaneously running threads in the program.





[Async in Rust]: https://aturon.github.io/apr/async-in-rust/chapter.html
[Rust]: https://www.rust-lang.org
[nightly Rust]: https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust
[futures]: https://crates.io/crates/futures
[tokio]: https://crates.io/crates/tokio

[1]: https://en.wikipedia.org/wiki/Asynchronous_I/O
[2]: https://github.com/rust-lang-nursery/wg-net/issues?q=is%3Aissue+is%3Aopen+label%3A%22WG+async%2Fawait%22
[3]: https://github.com/rust-lang/rust/issues/50547
[4]: https://en.wikipedia.org/wiki/Futures_and_promises
[5]: https://aturon.github.io/blog/2016/09/07/futures-design
[6]: http://aturon.github.io/blog/2016/09/07/futures-design/#what-worked-the-demand-driven-aka-readiness-based-approach
[7]: https://aturon.github.io/blog/2016/08/11/futures
[8]: https://aturon.github.io/apr/task-model/chapter.html
[9]: https://aturon.github.io/apr/futures/chapter.html
[10]: https://docs.rs/futures/0.1
[11]: https://docs.rs/futures/0.2
[12]: https://rust-lang-nursery.github.io/futures-api-docs
[13]: https://gist.github.com/Diggsey/6f924bf3f741bcdffd240faee102fe92
[14]: https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part-1-3k3
[15]: https://tokio.rs/docs/getting-started/futures
[16]: https://docs.rs/tokio
[17]: https://tokio.rs/docs/overview
[18]: https://github.com/nrc/apr-intro
