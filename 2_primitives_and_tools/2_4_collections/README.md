Step 2.4: Collections
=====================

While [Rust] allows you to build your own statically strongly typed collections it has already mature collections implementations for common cases.




## Standard collections

[Rust] has implementations for commonly used collections in its standard library ([`std::collections`] module). Trade-offs of provided implementations are suitable for 90% cases. Its documentation contains recommendation for [choosing concrete collection primitive depending on a problem][1]. Read through it to become familiar with provided primitives.




## Immutable collections

[Rust] has an excellent [im] crate which provides implementations of immutable collections.

Immutable collections are collections which preserve interface and behavior of its mutable analogues, but have different implementations which allow each piece of code to work with its own copy of the whole collection without worrying of accidentally changing items for others. This usually comes in price of performance, so immutable collection has [other performance guarantees][3] than mutable ones.

Read [im] crate [documentation][2] to understand the nature of immutable collections better and the motivation behind them.




## Task

Write a simple `UsersRepository` trait (interface) which supports 3 operations:
- returns single `User` by its ID;
- returns multiple `User`s by their IDs;
- return IDs of `User`s which `nickname` contains given string (search function).

Write a `UsersRepositoryMock` mock implementation of `UsersRepository` trait which allows in-place setup of returned values and is usable in test cases.

Write some tests for trivial scenarios which use `UsersRepository` to feel how your mock is handy and 
usable.





[`std::collections`]: https://doc.rust-lang.org/stable/std/collections
[Rust]: https://www.rust-lang.org
[im]: https://crates.io/crates/im

[1]: https://doc.rust-lang.org/stable/std/collections/index.html#when-should-you-use-which-collection
[2]: https://docs.rs/im
[3]: https://docs.rs/im/#lists
