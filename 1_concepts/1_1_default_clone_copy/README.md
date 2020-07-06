Step 1.1: Default values, cloning and copying
=============================================

__Estimated time__: 1 day




## Default values

[Rust] has a standard way to deal with default values of a type via [`Default`] trait. Read through [its official docs][`Default`] to understand the design.

It can be auto-derived, but only for a `struct` whose all members have [`Default`] implementations. It is implemented for a great many types in the standard library, and also used in a surprising number of places. So if your type has a value that can be construed as being "default", it is a good idea to implement this trait.

If you're not enough with [std] deriving capabilities for [`Default`], consider to use [smart-default] crate. An example is quite self-explanatory:
```rust
#[derive(SmartDefault)]
enum Foo {
    Bar,
    #[default]
    Baz {
        #[default = 12]
        a: i32,
        b: i32,
        #[default(Some(Default::default()))]
        c: Option<i32>,
        #[default(_code = "vec![1, 2, 3]")]
        d: Vec<u32>,
        #[default = "four"]
        e: String,
    },
    Qux(i32),
}
```

A great thing that having a [`Default`] implementation you can instantiate your `struct` with only the non-default values and have all other fields filled with default values:
```rust
let x = Foo { bar: baz, ..Default::default() };
```




## Cloning and copying

By default, all types in [Rust] follow ['move semantics'][1].

If you need a duplicate of a value, then its type should implement [`Clone`] trait (see [official docs][`Clone`]), and a duplicate is created by calling [`Clone`] methods __explicitly__. Cloning can be __either cheap or expensive__ operation depending on type semantics.

However, [`Copy`] marker trait (see [official docs][`Copy`]) enables 'copy semantics' for a type, so a value is __copied implicitly__ every time is passed. That's why copying must always perform a __simple bit-to-bit copy operation__.

[Official `Copy` docs][`Copy`] are quite explanatory about which types _should_ be [`Copy`] and which types _cannot_:

> Some types can't be copied safely. For example, copying `&mut T` would create an aliased mutable reference. Copying `String` would duplicate responsibility for managing the `String`'s buffer, leading to a double free.
> 
> Generalizing the latter case, any type implementing `Drop` can't be `Copy`, because it's managing some resource besides its own `size_of::<T>` bytes.

> Generally speaking, if your type can implement `Copy`, it should. Keep in mind, though, that implementing `Copy` is part of the public API of your type. If the type might become non-`Copy` in the future, it could be prudent to omit the `Copy` implementation now, to avoid a breaking API change.




## Task

- Create a `Point` type which represents a 2D point (`x` and `y` coordinates). This type has to be `Copy` and `Default`.
- Create a `Polyline` type which represents a non-empty set of `Point`s of unknown size. This type has to be `Clone` and non-`Default`. 





[`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[std]: https://doc.rust-lang.org/stable/std
[smart-default]: https://docs.rs/smart-default
[Rust]: https://www.rust-lang.org

[1]: https://stackoverflow.com/a/30290070/1828012
