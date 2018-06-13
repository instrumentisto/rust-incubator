Step 1.2: `Rc`/`Weak`: reference counting
=========================================

[Rust] ownership model allows only one owner of a value. However, there are situations when multiple ownership is required. This may be reached by wrapping a value into [`Rc`] ("reference counted") smart pointer, which __produces a new pointer to the same value each time is cloned__ and __destroys the value in a heap once there are no references left__.

The code below won't compile as `a` is owned by `x` and moved to a heap before is passed to `y`:
```rust
struct Val(u8);

fn main() {
    let a = Val(5);
    let x = Box::new(a);
    let y = Box::new(a);
}
```

```
error[E0382]: use of moved value: `a`
 --> src/main.rs:6:22
  |
5 |     let x = Box::new(a);
  |                      - value moved here
6 |     let y = Box::new(a);
  |                      ^ value used here after move
  |
  = note: move occurs because `a` has type `Val`, which does not implement the `Copy` trait
```

However, [`Rc`] allows that:
```rust
fn main() {
    let a = Rc::new(Val(5));
    let x = Rc::clone(&a);  // does not clone original value,
    let y = Rc::clone(&a);  // but rather produces new reference to it
}
```

The [`Rc`], however, __should be used wisely__ as __won't deallocate memory on references cycle__ which is exactly what __memory leak__ is. [Rust] is unable to prevent memory leaks at compile time (though makes hard to produce them). If it's still required to have references cycle, you should use [`Weak`] smart pointer ("weak reference") in combination with [`Rc`]. [`Weak`] allows to break references cycle as can refer to a value that has been dropped already (returns `None` in such case). 

For better understanding [`Rc`]/[`Weak`] purpose, design, limitations and use cases read through:
- [Rust Book: 15.4. Rc, the Reference Counted Smart Pointer][1]
- [Rust Book: 15.6. Reference Cycles Can Leak Memory][1]
- [Official `std:rc` docs][`std::rc`]




## Task

Write a simple program which reads a large file [`war_and_peace.pdf`](war_and_peace.pdf), places it to a heap and passes it to subsequent function calls (at least 5 times) with ownership and without cloning the whole file content each time.





[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`Weak`]: https://doc.rust-lang.org/std/rc/struct.Weak.html
[`std::rc`]: https://doc.rust-lang.org/std/rc
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/book/second-edition/ch15-04-rc.html
[2]: https://doc.rust-lang.org/book/second-edition/ch15-06-reference-cycles.html
