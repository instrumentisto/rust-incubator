Step 3.10: Multithreading and parallelism
=========================================

__Estimated time__: 1 day

One of main [Rust]'s design goals is a [concurrency][1]. [Rust] has a [strong opinion][2] about that, while allows different concurrent models to coexist.




## Threads

[Rust] has built-in support for [native threads][3] in a form of [`std::thread`] module of standard library.

Traditionally, [threads][3] are used for solving [CPU-bound] problems, as they allow to execute tasks in parallel. However, in practice, threads are often used to solve [I/O-bound] problems too, especially when [asynchronous I/O][4] is not supported well (which is true for [Rust] `std` library at the moment).

[crossbeam] crate also provides implementation of [scoped threads][5], which allow to borrow values from a stack.

For better understanding [Rust] threads design, concepts, usage, and features (especially [TLS][4] is important and widely used one), read through the following articles:
- [Rust Book: 16.1. Using Threads to Run Code Simultaneously][6]
- [Rust By Example: 20.1. Threads][7]
- [Official `std::thread` docs][`std::thread`]




## Synchronization

The [threads synchronization][11] is a wide topic, but generally it's done via [atomic operations][12], shared state with an [exclusive access][13], or by [threads communication][14]. [Rust] has built-in support for all of them.

[Atomic operations][12] are represented by [`std::sync::atomic`] module of standard library.

[Exclusive access][13] may be controlled via primitives of [`std::sync`] module of standard library.

Threads communication is commonly represented via [channels][14] and is implemented in [`std::sync::mpsc`] module of standard library. 

Despite that, there is also [crossbeam] crate, which provides more feature-rich and optimized concurrency and synchronization primitives. The most notable is [crossbeam-channel] as [an enhancement][15] of `std` channel implementations.

For better understanding and familiarity with [Rust] synchronization primitives design, concepts, usage, and features, read through the following articles:
- [Rust Book: 16.2. Using Message Passing to Transfer Data Between Threads][16]
- [Rust Book: 16.3. Shared-State Concurrency][13]
- [Rust Blog: Fearless Concurrency with Rust][2]
- [Official `std::sync` docs][`std::sync`]
- [Official `std::sync::atomic` docs][`std::sync::atomic`]
- [Official `std::sync::mpsc` docs][`std::sync::mpsc`]
- [Official `crossbeam-channel` crate docs][crossbeam-channel]
- [Carl Fredrik Samson: Explaining Atomics in Rust][26]




## Parallelism

The important concept to understand is [how concurrency and parallelism differ][21].

[Rust] ecosystem has support for parallelism in form of [rayon] crate, which makes it easy to convert a sequential iterator to _execute in parallel threads_.

Another way to perform parallel data processing _without using threads_ is [SIMD] instructions usage. If an algorithm is parallelizable enough, applying [SIMD] instructions may [increase performance drastically][24]. [Rust] ecosystem provides basic support for [SIMD] instructions in a form of [packed_simd] crate.

For better understanding and familiarity with parallelism in [Rust], read through the following articles:
- [Official `rayon` crate docs][rayon]
- [`rayon` crate FAQ][22]
- [`rayon` crate demos][23]
- [Rust Edition Guide: 3.9. SIMD for faster computing][25]
- [Official `packed_simd` crate docs][packed_simd]




## Task

Write a program with the following workflow:
- `Producer` is a separate thread, which continuously generates square matrixes of random `u8` elements and size `4096`.
- `Consumer` is a separate thread, which takes a generated matrix, counts sum of all its elements and prints the sum to STDOUT.
- There are only 1 `Producer` and 2 `Consumer`s.
- Counting sum of matrix elements should be parallelized. 





[CPU-bound]: https://en.wikipedia.org/wiki/CPU-bound
[crossbeam]: https://docs.rs/crossbeam
[crossbeam-channel]: https://docs.rs/crossbeam-channel
[I/O-bound]: https://en.wikipedia.org/wiki/I/O_bound
[packed_simd]: https://docs.rs/packed_simd
[rayon]: https://docs.rs/rayon
[Rust]: https://www.rust-lang.org
[SIMD]: https://en.wikipedia.org/wiki/SIMD
[`std::sync`]: https://doc.rust-lang.org/std/sync/index.html
[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/index.html
[`std::sync::mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[`std::thread`]: https://doc.rust-lang.org/std/thread/index.html

[1]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[2]: https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
[3]: https://en.wikipedia.org/wiki/Thread_(computing)
[4]: https://en.wikipedia.org/wiki/Asynchronous_I/O
[5]: https://docs.rs/crossbeam/0.7.1/crossbeam/thread/index.html
[6]: https://doc.rust-lang.org/book/ch16-01-threads.html
[7]: https://doc.rust-lang.org/rust-by-example/std_misc/threads.html
[8]: https://doc.rust-lang.org/std/thread/index.html#thread-local-storage
[11]: https://en.wikipedia.org/wiki/Synchronization_(computer_science)#Thread_or_process_synchronization
[12]: https://en.wikipedia.org/wiki/Linearizability
[13]: https://doc.rust-lang.org/book/ch16-03-shared-state.html
[14]: https://en.wikipedia.org/wiki/Channel_(programming)
[15]: https://stjepang.github.io/2017/08/13/designing-a-channel.html
[16]: https://doc.rust-lang.org/book/ch16-02-message-passing.html
[21]: https://stackoverflow.com/a/1050257/1828012
[22]: https://github.com/rayon-rs/rayon/blob/master/FAQ.md
[23]: https://github.com/rayon-rs/rayon/tree/master/rayon-demo
[23]: https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html
[24]: https://branchfree.org/2019/02/25/paper-parsing-gigabytes-of-json-per-second
[25]: https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html
[26]: https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust
