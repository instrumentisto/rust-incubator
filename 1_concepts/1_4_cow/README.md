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

To better understand [`Cow`]'s purpose, design, limitations and use cases, read through:
- [Official `Cow` docs][`Cow`]
- [Pascal Hertleif: The Secret Life of Cows][1]
- [Yashodhan Joshi: Using `Cow` in Rust for efficient memory utilization][3]
- [Konstantin Grechishchev: 6 things you can do with the Cow 🐄 in Rust 🦀][2]




## Alternative implementations

[`beef`] crate provides alternative `Cow` types, being faster and leaner.

> There are two versions of `Cow` exposed by this crate:
>
> - `beef::Cow` is 3 words wide: pointer, length, and capacity. It stores the ownership tag in capacity.
> - `beef::lean::Cow` is 2 words wide, storing length, capacity, and the ownership tag all in one word.
> 
> Both versions are leaner than the `std::borrow::Cow`:
> ```rust
> use std::mem::size_of;
> 
> const WORD: usize = size_of::<usize>();
> 
> assert_eq!(size_of::<std::borrow::Cow<str>>(), 4 * WORD);
> assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
> 
> // Lean variant is two words on 64-bit architecture
> #[cfg(target_pointer_width = "64")]
> assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);
> ```

Read implementation details and design insights in [its README][4].




## Task

Write a simple program which prints out the path to its configuration file. The path should be detected with the following precedence:
1. default path is `/etc/app/app.conf`;
2. if `APP_CONF` env var is specified (and not empty) then use it with higher priority than default;
3. if `--conf` command line argument is specified (error if empty) then use it with the highest priority.

If neither `APP_CONF` env var nor `--conf` command line argument is specified, then no allocation should happen for path detection.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is [`Cow`]? How it works?
- When [`Cow`] is useful and why? Give some meaningful examples.




[`beef`]: https://docs.rs/beef
[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[Rust]: https://www.rust-lang.org

[1]: https://deterministic.space/secret-life-of-cows.html
[2]: https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55
[3]: https://blog.logrocket.com/using-cow-rust-efficient-memory-utilization
[4]: https://github.com/maciejhirsz/beef#how-does-it-work
