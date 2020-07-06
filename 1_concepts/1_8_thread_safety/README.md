Step 1.8: Thread safety
=======================

__Estimated time__: 1 day

[Rust] has [`Send`] and [`Sync`] marker traits which are fundamental for concurrency and thread safety story in [Rust] and represent one of [fearless concurrency][2] corner stones (which allow to [avoid data races][1] at compile time).

For better understanding [`Send`]/[`Sync`] purpose, design, limitations and use cases, read through the following articles:
- [Official `Send` docs][`Send`]
- [Official `Sync` docs][`Sync`]
- [Rust Book: 16.4. Extensible Concurrency with the Sync and Send Traits][3]
- [Rustonomicon: 8.2. Send and Sync][4]
- [Huon Wilson: Some notes on Send and Sync][5]




## Task

Implement the following types, which meet conditions:
1. `OnlySync` is `Sync`, but `!Send`.
2. `OnlySend` is `Send`, but `!Sync`.
3. `SyncAndSend` is both `Sync` and `Send`.
4. `NotSyncNotSend` is both `!Sync` and `!Send`.

All inner details of implementation  are on your choice.

Play with these types from multiple threads to see how compile time [fearless concurrency][2] works in practice.





[Rust]: https://www.rust-lang.org
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html

[1]: https://doc.rust-lang.org/nomicon/races.html
[2]: https://doc.rust-lang.org/book/ch16-00-concurrency.html
[3]: https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html
[4]: https://doc.rust-lang.org/stable/nomicon/send-and-sync.html
[5]: http://huonw.github.io/blog/2015/02/some-notes-on-send-and-sync
