Step 2.12: Threads, synchronization and parallelism
===================================================

One of main [Rust]'s design goals is a [concurrency][1]. [Rust] has a [strong opinion][2] about that while allows different concurrent models to co-exist.




## Threads

[Rust] has built-in support for [native threads][3] in form of [`std::thread`] module of standard library.

Read through [its documentation][`std::thread`] carefully to understand the design of [threads][3] in [Rust] and their features (especially [TLS][4] is important and widely used one). 

Also, the following articles are useful:
- [Rust Book: 16.1. Using Threads to Run Code Simultaneously][5]




## Synchronization

The [threads synchronization][6] is a wide topic, but generally it's done via [atomic operations][7], shared state with an [exclusive access][8], or by [threads communication][9]. [Rust] has built-in support for all these kinds.

[Atomic operations][7] are represented by [`std::sync::atomic`] module of standard library.

[Exclusive access][8] may be controlled via primitives of [`std::sync`] module of standard library. This is described quite well in the following articles:
- [Rust Book: 16.3. Shared-State Concurrency][8]

Threads communication is commonly represent via [channels][9] and is implemented in [`std::sync::mpsc`] module of standard library.

Despite that there is also the [crossbeam] crate which provides more feature-rich and optimized concurrency and synchronization primitives. The most notable is [crossbeam-channel] as [an enhancement][10] of a standard library channel implementation.




## Parallelism

To use the correct tool in the right place the important concept to understand is [how concurrency and parallelism differ][11].

[Rust] has support for parallelism in form of [rayon] crate. Read through [its documentation][12] to understand better the usage and the design of this crate. Look at [demos][13] to see how it can be applied.




## Task

Implement the following lifecycle:
- `Producer` continuously generates square matrixes of random `u8` elements and size `4096`.
- `Consumer` takes generated matrix, counts sum of all its elements and prints the sum to STDOUT.
- There are only 1 `Producer` and 2 `Consumer`s. Each has its own separate thread.
- Counting sum of matrix elements must be parallelized. 





[`std::sync`]: https://doc.rust-lang.org/stable/std/sync/index.html
[`std::sync::atomic`]: https://doc.rust-lang.org/stable/std/sync/atomic/index.html
[`std::sync::mpsc`]: https://doc.rust-lang.org/stable/std/sync/mpsc/index.html
[`std::thread`]: https://doc.rust-lang.org/stable/std/thread/index.html
[Rust]: https://www.rust-lang.org
[crossbeam]: https://crates.io/crates/crossbeam
[crossbeam-channel]: https://crates.io/crates/crossbeam-channel
[rayon]: https://crates.io/crates/rayon

[1]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[2]: https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
[3]: https://en.wikipedia.org/wiki/Thread_(computing)
[4]: https://doc.rust-lang.org/stable/std/thread/index.html#thread-local-storage
[5]: https://doc.rust-lang.org/book/second-edition/ch16-01-threads.html
[6]: https://en.wikipedia.org/wiki/Synchronization_(computer_science)#Thread_or_process_synchronization
[7]: https://en.wikipedia.org/wiki/Linearizability
[8]: https://doc.rust-lang.org/book/second-edition/ch16-03-shared-state.html
[9]: https://en.wikipedia.org/wiki/Channel_(programming)
[10]: https://stjepang.github.io/2017/08/13/designing-a-channel.html
[11]: https://stackoverflow.com/a/1050257/1828012
[12]: https://docs.rs/rayon
[13]: https://github.com/rayon-rs/rayon/tree/master/rayon-demo
