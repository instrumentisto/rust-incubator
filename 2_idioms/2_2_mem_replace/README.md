Step 2.2: Swapping values with `mem::replace`
=============================================

__Estimated time__: 1 day

As [Rust] implies [move semantics][1] by default and quite strict [borrowing rules][2], often, there are situations (especially, with large `struct`s and `enum`s) where mutating value in-place or values swapping may not be allowed by borrow checker, which is quite confusing and leads to doing needless clones (so providing redudant performance costs). For example:
```rust
impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        // error: cannot move out of dereference of `&mut`-pointer
        let buf = self.buf;
        self.buf = Vec::new();
        buf
    }
}
```

A neat and need-to-know trick in such situations is to use [`mem::replace`] (or [`mem::swap`]). It allows to swap two values of the same type without moving things around, partial destructuring and references mess. So, the example above is simply turns into:
```rust
impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        mem::replace(&mut self.buf, Vec::new())
    }
}
```

For better understanding [`mem::replace`] and [`mem::swap`] purpose, design, limitations and use cases, read through the following articles:
- [Official `mem::replace` docs][`mem::replace`]
- [Official `mem::swap` docs][`mem::swap`]
- [Karol Kuczmarski: Moving out of a container in Rust][4]

Some examples of useful applying these functions are described below.




## Keeping owned values in changed enums

This situation has detailed explanation in the following article:
- [Rust Design Patterns: `mem::replace` to keep owned values in changed enums][3]

> The borrow checker won't allow us to take out `name` of the enum (because _something_ must be there). We could of course `.clone()` name and put the clone into our `MyEnum::B`, but that would be an instance of the "Clone to satisfy the borrow checker" antipattern. Anyway, we can avoid the extra allocation by changing `e` with only a mutable borrow.
> 
> `mem::replace` lets us swap out the value, replacing it with something else. In this case, we put in an empty `String`, which does not need to allocate. As a result, we get the original `name` _as an owned value_. We can then wrap this in another enum.

```rust
enum MyEnum {
    A { name: String },
    B { name: String },
}

fn swizzle(e: &mut MyEnum) {
    use self::MyEnum::*;
    *e = match *e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { ref mut name } => B { name: mem::replace(name, String::new()) },
        B { ref mut name } => A { name: mem::replace(name, String::new()) },
    }
}
```

> Look ma, no allocation! Also you may feel like Indiana Jones while doing it.




## Mutating embedded collection

Consider the following situation:
```rust
struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}

impl Names {
    fn apply_exclusions(&mut self) {
        self.exclusions.drain(..).for_each(|name| {
            self.names.remove(&name);
        })
    }
}
```
which does not compile due to 2 mutable borrows:
```rust
error[E0500]: closure requires unique access to `self` but it is already borrowed
  --> src/main.rs:10:44
   |
10 |         self.exclusions.drain(..).for_each(|name| {
   |         ---------------           -------- ^^^^^^ closure construction occurs here
   |         |                         |
   |         |                         first borrow later used by call
   |         borrow occurs here
11 |             self.names.remove(&name);
   |             ---- second borrow occurs due to use of `self` in closure
```

Using [`mem::replace`] here allows us to avoid 2 mutable borrows problem at almost no cost (`HashSet::new()` is no-op), by swapping out the value in a temporary variable:
```rust
impl Names {
    fn apply_exclusions(&mut self) {
        let mut names = mem::replace(&mut self.names, HashSet::new());
        self.exclusions.drain(..).for_each(|name| {
            names.remove(&name);
        });
        mem::replace(&mut self.names, names);
    }
}
```




## Task

Improve and optimize the code contained in [this step's crate](src/main.rs) to cut off redudant performance costs.





[`mem::replace`]: https://doc.rust-lang.org/std/mem/fn.replace.html
[`mem::swap`]: https://doc.rust-lang.org/std/mem/fn.swap.html
[Rust]: https://www.rust-lang.org

[1]: https://stackoverflow.com/a/30290070/1828012
[2]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
[3]: https://github.com/rust-unofficial/patterns/blob/master/idioms/mem-replace.md
[4]: http://xion.io/post/code/rust-move-out-of-container.html
