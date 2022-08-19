Step 3.6: Serialization and deserialization
===========================================

__Estimated time__: 1 day




## [`serde`] framework

[Rust] ecosystem has the well-known [`serde`] crate, which provides a common (standard, de facto) approach and toolset for serialization and deserialization.

The sweet part is that [`serde`] __does not rely on a runtime reflection__ mechanism and uses trait implementation for each type, so __eliminates most runtime costs__ and in most cases __makes serialization as performant as handwritten serializer for a particular case__, yet __remains ergonomic due to [automatic code deriving][1]__.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
```

[`serde`] by itself represents only a universal serialization frontend, which can be backed by actual implementation for any format. There are already [implemented backends for most used formats][2], and you're free to [implement backend for your own format][3] if it's not implemented yet. 

For better understanding and familiarity with [`serde`]'s design, concepts, usage, and features (like [zero-copy deserialization][5]), read through the following articles:
- [Official `serde` crate guide][0]
- [Official `serde` crate docs][`serde`]
- [Official `serde_json` crate docs][`serde_json`]
- [Owen Gage: Understanding Rust's serde using macro expansion][6]
- [Owen Gage: Exploring serde's data model with a toy deserializer][7]
- [Owen Gage: A look at serde-json][11]
- [Manish Goregaokar: Not a Yoking Matter (Zero-Copy #1)][12]
- [Manish Goregaokar: Zero-Copy All the Things! (Zero-Copy #2)][13]
- [Manish Goregaokar: So Zero It's ... Negative? (Zero-Copy #3)][14]


### Extras

Being the de facto ecosystem standard, [`serde`] crate itself is quite conservative about stability guarantees, so often may feel lacking obvious features. Therefore, additional ecosystem crates are worth considering, which extend [`serde`] capabilities, being built on top of its machinery:
- [`erased-serde`] crate, providing type-erased versions of `serde`’s `Serialize`, `Serializer` and `Deserializer` traits that can be used as [trait objects][9].
- [`serde_state`] crate, extending the normal `Deserialize` and `Serialize` traits to allow state to be passed to every value which is serialized or deserialized.
- [`serde_repr`] crate, deriving `serde`'s `Serialize` and `Deserialize` traits in a way that delegates to the underlying repr of a C-like enum.
- [`serde_with`] crate, providing custom de/serialization helpers to use in combination with [`serde`’s `with`-annotation][8] and with the improved `serde_as`-annotation.
- [`serde_valid`] crate, enabling [JSON Schema][10] based validation. 




## Task

Write a program which deserializes the [following JSON](request.json) into a static `Request` type and prints out its serialization in a YAML and TOML formats. Consider to choose correct types for data representation.

Prove your implementation correctness with tests.




[`erased-serde`]: https://docs.rs/erased-serde
[`serde`]: https://docs.rs/serde
[`serde_json`]: https://docs.rs/serde_json
[`serde_repr`]: https://docs.rs/serde_repr
[`serde_state`]: https://docs.rs/serde_state
[`serde_valid`]: https://docs.rs/serde_valid
[`serde_with`]: https://docs.rs/serde_with
[Rust]: https://www.rust-lang.org

[0]: https://serde.rs
[1]: https://serde.rs/derive.html
[2]: https://serde.rs/index.html#data-formats
[3]: https://serde.rs/data-format.html
[4]: https://serde.rs/examples.html
[5]: https://serde.rs/lifetimes.html#understanding-deserializer-lifetimes
[6]: https://owengage.com/writing/2021-07-23-serde-expand
[7]: https://owengage.com/writing/2021-08-14-serde-toy
[8]: https://serde.rs/field-attrs.html#with
[9]: https://doc.rust-lang.org/book/trait-objects.html
[10]: https://json-schema.org
[11]: https://owengage.com/writing/2022-07-22-a-look-at-serde-json
[12]: https://manishearth.github.io/blog/2022/08/03/zero-copy-1-not-a-yoking-matter
[13]: https://manishearth.github.io/blog/2022/08/03/zero-copy-2-zero-copy-all-the-things
[14]: https://manishearth.github.io/blog/2022/08/03/zero-copy-3-so-zero-its-dot-dot-dot-negative
