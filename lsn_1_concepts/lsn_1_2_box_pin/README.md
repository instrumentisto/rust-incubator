Step 1.2: Boxing and pinning
============================

__Estimated time__: 1 day




## Boxing

[`Box`] is a pointer that owns heap-allocated data. This is the most common and simples form of [heap] allocation in [Rust].

It's more idiomatic to use references (`&T`/`&mut T`) for pointing to the data, however they often are coming with lifetimes complexity. [`Box`] allows to avoid this complexity at the cost of heap allocation.

For better understanding [`Box`] purpose, design, limitations and use cases read through:
- [Rust Book: 15.1. Using Box to Point to Data on the Heap][1]
- [Official `std::boxed` docs][`std::boxed`]




## Pinning

It is sometimes useful to have objects that are guaranteed to not move, in the sense that their placement in memory does not change, and can thus be relied upon. A prime example of such a scenario would be building self-referential structs, since moving an object with pointers to itself will invalidate them, which could cause undefined behavior.

[`Pin<P>`][`Pin`] ensures that the pointee of any pointer type `P` has a stable location in memory, meaning it cannot be moved elsewhere and its memory cannot be deallocated until it gets dropped. We say that the pointee is "pinned".

However, many types are always freely movable, even when pinned, because they do not rely on having a stable address. This includes all the basic types (like `bool`, `i32`, references) as well as types consisting solely of these types. Types that do not care about pinning implement the [`Unpin`] marker trait, which cancels the effect of [`Pin`]. For `T: Unpin`, `Pin<Box<T>>` and `Box<T>` function identically, as do `Pin<&mut T>` and `&mut T`.

Note, that pinning and [`Unpin`] only affect the pointed-to type `P::Target`, not the pointer type `P` itself that got wrapped in `Pin<P>`. For example, whether or not `Box<T>` is `Unpin` has no effect on the behavior of `Pin<Box<T>>` (here, `T` is the pointed-to type).

For better understanding [`Pin`] purpose, design, limitations and use cases read through:
- [Official `std::pin` docs][`std::pin`]
- [Reddit: Pinned objects ELI5?][1]




## Task

Given the following traits:
```rust
trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}
```
```rust
trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}
```

Implement them for the following types: `Box<T>`, `Rc<T>`, `Vec<T>`, `String`, `&[u8]`, `T`.





[heap]: https://en.wikipedia.org/wiki/Memory_management#HEAP
[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Pin`]: https://doc.rust-lang.org/std/pin/struct.Pin.html
[Rust]: https://www.rust-lang.org
[`std::boxed`]: https://doc.rust-lang.org/std/boxed/index.html
[`std::pin`]: https://doc.rust-lang.org/std/pin/index.html
[`Unpin`]: https://doc.rust-lang.org/std/marker/trait.Unpin.html

[1]: https://doc.rust-lang.org/book/ch15-01-box.html
[2]: https://www.reddit.com/r/rust/comments/9akmqv/pinned_objects_eli5/
