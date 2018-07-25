Step 1.4: `Cow`: clone-on-write
===============================

[Rust] has a [`Cow`] (clone-on-write) smart pointer in its standard library. Understanding how to use it is __essential to write idiomatic and ergonomic__ [Rust] code.

In a nutshell: it allows to combine usage of owned and borrowed data in a single abstraction, which __leads to better ergonomics and minimize performance penalties asap__ (as much as possible).

```rust
use std::borrow::Cow;

fn describe(error: &Error) -> Cow<'static, str> {
    match *error {
        // Returning &'str - a borrowed reference to static str.
        Error::NotFound => "Error: Not found".into(),
        
        // Returning String - an owned String allocated in heap.
        Error::Custom(e) => format!("Error: {}", e).into(),
    }
}
```

To understand the purpose, design and usage of [`Cow`] read through the following article, which addresses those questions quite well:
- [The Secret Life of Cows]

Also, be friendly with [official `Cow` docs][`Cow`].




## Task

Write a simple program which prints out the path to its configuration file. The path should be detected with the following precedence:
1. Default path is `/etc/app/app.conf`.
2. If `APP_CONF` env var is specified (and not empty) then use it.
3. If `--conf` command line argument is specified (error if empty) then use it.

Path detection must be implemented as a separate function and covered with tests.





[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[Rust]: https://www.rust-lang.org
[The Secret Life of Cows]: https://deterministic.space/secret-life-of-cows.html
