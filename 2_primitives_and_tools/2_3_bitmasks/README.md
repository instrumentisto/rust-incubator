Step 2.3: Bitmasks
==================

[Bitmask][bitmasks] in programming is a common way to store multiple bit options in a single integer value. To extract or modify bit options the bitwise operators are used.

[Rust] has the well-known [bitflags] crate which provides ergonomic tools to deal with [bitmasks].

```rust
#[macro_use]
extern crate bitflags;

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

fn main() {
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC);   // union
    assert_eq!((e1 & e2), Flags::C);     // intersection
    assert_eq!((e1 - e2), Flags::A);     // set difference
    assert_eq!(!e2, Flags::A);           // set complement
}
```




## Task

Implement a dummy analogue of [PHP `json_encode()`][1] function which accepts [options bitmask][2] as a second parameter and returns its readable string representation instead of actual JSON.

The point is to recreate all [bitmask options][2] used by `json_encode()` function in PHP.

Prove your implementation correctness with tests.





[bitflags]: https://crates.io/crates/bitflags
[bitmasks]: https://en.wikipedia.org/wiki/Mask_(computing)
[Rust]: https://www.rust-lang.org

[1]: http://php.net/manual/en/function.json-encode.php
[2]: http://php.net/manual/en/function.json-encode.php#refsect1-function.json-encode-parameters
