Step 1.8: `Send`/`Sync`: thread safety
======================================

[Rust] has [`Send`] and [`Sync`] marker traits which are fundamental for concurrency and thread safety story in [Rust] and represent one of [fearless concurrency][2] corner stones (which allow to [avoid races][1] at compile time).

For better understanding and use cases of [`Send`]/[`Sync`], read through their documentation and the following articles:
- [Rust Book: 16.4. Extensible Concurrency with the `Sync` and `Send` Traits][3]
- [Rustonomicon: 8.2. Send and Sync][4]
- [Some notes on Send and Sync][5]




## Task

Implement the following types that follow conditions:
1. `OnlySync` is `Sync`, but `!Send`.
2. `SyncAndSend` is both `Sync` and `Send`.
3. `NotSync` is `!Sync`.

All implementation inner details are on your choice.

Play with these types from multiple threads to see how [fearless concurrency][2] works in practice.





[`Send`]: https://doc.rust-lang.org/stable/std/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/stable/std/marker/trait.Sync.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/stable/nomicon/races.html
[2]: https://doc.rust-lang.org/book/second-edition/ch16-00-concurrency.html
[3]: https://doc.rust-lang.org/book/second-edition/ch16-04-extensible-concurrency-sync-and-send.html
[4]: https://doc.rust-lang.org/stable/nomicon/send-and-sync.html
[5]: http://huonw.github.io/blog/2015/02/some-notes-on-send-and-sync
