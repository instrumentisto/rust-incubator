Step 1.4: Clone-on-write
========================

__Estimated time__: 1 day




## Clone-on-write

[Rust] has a [`Cow`] (clone-on-write) smart pointer in its standard library. Understanding how to use it is _essential to write idiomatic and ergonomic_ [Rust] code.

In a nutshell: 
- it allows to combine usage of owned and borrowed data in a single abstraction, which __leads to better ergonomics and minimize performance penalties asap__ (as much as possible);
- it encloses and provides immutable access to borrowed data, and __clones the data lazily when mutation or ownership is required__.

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

For better understanding [`Cow`] purpose, design, limitations and use cases read through:
- [Pascal Hertleif: The Secret Life of Cows][1]
- [Official `Cow` docs][`Cow`]




## Task

Write a simple program which prints out the path to its configuration file. The path should be detected with the following precedence:
1. default path is `/etc/app/app.conf`;
2. if `APP_CONF` env var is specified (and not empty) then use it;
3. if `--conf` command line argument is specified (error if empty) then use it.

If neither `APP_CONF` env var nor `--conf` command line argument is specified, then no allocation should happen for path detection.





[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[Rust]: https://www.rust-lang.org

[1]: https://deterministic.space/secret-life-of-cows.html
