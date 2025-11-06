Step 3.10: Multithreading and parallelism
=========================================

__Estimated time__: 1 day

One of main [Rust]'s design goals is a [concurrency][1]. [Rust] has a [strong opinion][2] about that, while allows different concurrent models to coexist.




## Threads

[Rust] has built-in support for [native threads][3] in form of the [`std::thread`] module of its standard library.

Traditionally, [threads][3] are used for solving [CPU-bound] problems, as they allow to execute tasks in parallel. However, in practice, threads are often used to solve [I/O-bound] problems too, especially when [asynchronous I/O][4] is not supported well (which is true for [Rust] `std` library at the moment).

[`crossbeam`] crate also provides implementation of [scoped threads][5], which allow to borrow values from a stack. They are also available in form of [`std::thread::scope`], as of [Rust] 1.63. 

To better understand [Rust]'s threads design, concepts, usage, and features (especially [TLS][4] is important and widely used one), read through:
- [Rust Book: 16.1. Using Threads to Run Code Simultaneously][6]
- [Rust By Example: 20.1. Threads][7]
- [Official `std::thread` docs][`std::thread`]
- [Nicky Meuleman: Multithreading in Rust][29]




## Synchronization

The [threads synchronization][11] is a wide topic, but generally it's done via [atomic operations][12], shared state with an [exclusive access][13], or by [threads communication][14]. [Rust] has built-in support for all of them.

[Atomic operations][12] are represented by [`std::sync::atomic`] module of [Rust] standard library (and, additionally, [`atomic`] crate).

[Exclusive access][13] may be controlled via primitives of [`std::sync`] module of [Rust] standard library.

Threads communication is commonly represented via [channels][14] and is implemented in [`std::sync::mpsc`] module of [Rust] standard library. 

Despite that, there is also the [`crossbeam`] crate, providing more feature-rich and optimized concurrency and synchronization primitives. The most notable is [`crossbeam-channel`] as [an enhancement][15] of `std` channel implementations.

To better understand and be familiar with [Rust]'s synchronization primitives design, concepts, usage, and features, read through:
- [Rust Book: 16.2. Using Message Passing to Transfer Data Between Threads][16]
- [Rust Book: 16.3. Shared-State Concurrency][13]
- [Rust Blog: Fearless Concurrency with Rust][2]
- [Official `std::sync` docs][`std::sync`]
- [Official `std::sync::atomic` docs][`std::sync::atomic`]
- [Official `std::sync::mpsc` docs][`std::sync::mpsc`]
- [Official `atomic` crate docs][`atomic`]
- [Official `crossbeam-channel` crate docs][`crossbeam-channel`]
- [Nicky Meuleman: Multithreading in Rust][29]
- [Carl Fredrik Samson: Explaining Atomics in Rust][26]
- [Gray Olson: The plight of the misunderstood memory ordering][37]
- [Aleksey Kladov: Mutexes Are Faster Than Spinlocks][27]
- [Mara Bos: Comparing Rust's and C++'s Concurrency Library][31]
- [Mahmoud Al-Qudsi: Implementing truly safe semaphores in rust][32]
- [Michael Snoyman: My Best and Worst Deadlock in Rust][35]
- [Cuong Le: Inside Rust's std and parking_lot mutexes - who wins?][38]




## Parallelism

The important concept to understand is [how concurrency and parallelism differ][21].

[Rust] ecosystem has support for parallelism in form of [`rayon`], [`dpc-pariter`] and [`fork_union`] crates, which make it easy to convert a sequential iterator to _execute in parallel threads_.

Another way to perform parallel data processing _without using [threads][3]_ is [SIMD] instructions usage. If an algorithm is parallelizable enough, applying [SIMD] instructions may [increase performance drastically][24]. [Rust] ecosystem provides basic support for [SIMD] instructions in a form of [`packed_simd`] crate.

To better understand and be familiar with parallelism in [Rust], read through:
- [Nicky Meuleman: Concurrent vs parallel][28]
- [Official `rayon` crate docs][`rayon`]
- [`rayon` crate FAQ][22]
- [`rayon` crate demos][23]
- [Kofi Otuo: Implementing data parallelism with Rayon Rust][34]
- [Dawid Ciężarkiewicz: Adding parallelism to your Rust iterators with `dpc-pariter`][30]
- [Official `dpc-pariter` crate docs][`dpc-pariter`]
- [Ash Vardanian: Fork Union: Beyond OpenMP in C++ and Rust?][36]
- [Official `fork_union` crate docs][`fork_union`]
- [Rust Edition Guide: 3.9. SIMD for faster computing][25]
- [Official `packed_simd` crate docs][`packed_simd`]
- [vgatherps: Parsing numbers into base-10 decimals with SIMD][33]
- [Sergey Davidoff: The state of SIMD in Rust in 2025][39]




## Task

Write a program with the following workflow:
- `Producer` is a separate thread, which continuously generates square matrixes of random `u8` elements and size `4096`.
- `Consumer` is a separate thread, which takes a generated matrix, counts sum of all its elements and prints the sum to STDOUT.
- There are only 1 `Producer` and 2 `Consumer`s.
- Counting sum of matrix elements should be parallelized.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is concurrency? What is parallelism? How do they relate to each other and how do they differ?
- How parallelism is represented in [Rust]? Which are common crates for using it?
- What are the main ways of threads synchronization in [Rust]? Which advantages and disadvantages does each one have? What are the use-cases for each one?




[`atomic`]: https://docs.rs/atomic
[`crossbeam`]: https://docs.rs/crossbeam
[`crossbeam-channel`]: https://docs.rs/crossbeam-channel
[`dpc-pariter`]: https://docs.rs/dpc-pariter
[`fork_union`]: https://docs.rs/fork_union
[`packed_simd`]: https://docs.rs/packed_simd
[`rayon`]: https://docs.rs/rayon
[`std::sync`]: https://doc.rust-lang.org/std/sync/index.html
[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/index.html
[`std::sync::mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[`std::thread`]: https://doc.rust-lang.org/std/thread/index.html
[`std::thread::scope`]: https://doc.rust-lang.org/std/thread/fn.scope.html
[CPU-bound]: https://en.wikipedia.org/wiki/CPU-bound
[I/O-bound]: https://en.wikipedia.org/wiki/I/O_bound
[Rust]: https://www.rust-lang.org
[SIMD]: https://en.wikipedia.org/wiki/SIMD

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
[15]: ../../archive/Stjepan_Glavina_Designing_a_channel.md
[16]: https://doc.rust-lang.org/book/ch16-02-message-passing.html
[21]: https://stackoverflow.com/a/1050257/1828012
[22]: https://github.com/rayon-rs/rayon/blob/master/FAQ.md
[23]: https://github.com/rayon-rs/rayon/tree/master/rayon-demo
[23]: https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html
[24]: https://branchfree.org/2019/02/25/paper-parsing-gigabytes-of-json-per-second
[25]: https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html
[26]: https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust
[27]: https://matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html
[28]: https://nickymeuleman.netlify.app/garden/concurrent-vs-parallel
[29]: https://nickymeuleman.netlify.app/blog/multithreading-rust
[30]: https://dpc.pw/adding-parallelism-to-your-rust-iterators
[31]: https://blog.m-ou.se/rust-cpp-concurrency
[32]: https://neosmart.net/blog/implementing-truly-safe-semaphores-in-rust/
[33]: https://vgatherps.github.io/2022-11-28-dec
[34]: https://blog.logrocket.com/implementing-data-parallelism-rayon-rust
[35]: https://www.snoyman.com/blog/2024/01/best-worst-deadlock-rust
[36]: https://ashvardanian.com/posts/beyond-openmp-in-cpp-rust
[37]: https://www.grayolson.me/blog/posts/misunderstood-memory-ordering
[38]: https://blog.cuongle.dev/p/inside-rusts-std-and-parking-lot-mutexes-who-win
[39]: https://shnatsel.medium.com/the-state-of-simd-in-rust-in-2025-32c263e5f53d
