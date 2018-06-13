Step 1.5: `Default`: default values
===================================

[Rust] has a standard way to deal with default values of type via [`Default`] trait. Read through [its official docs][`Default`] to understand the design.

It can be auto-derived, but only for `struct` whose members all have a [`Default`] implementations. It is implemented for a great many types in the standard library, and also used in a surprising number of places. So if your type has a value that can be construed as being “default”, it is a good idea to implement this trait.

A great thing that having a [`Default`] implementation you can instantiate your `struct` with only the non-default values and have all other fields filled with default values:
```rust
let x = Foo { bar: baz, ..Default::default() };
```




## Task

Take your code from [Step 1.1] and improve with a [`Default`] trait usage.





[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[Rust]: https://www.rust-lang.org

[Step 1.1]: ../1_1_type_safety
