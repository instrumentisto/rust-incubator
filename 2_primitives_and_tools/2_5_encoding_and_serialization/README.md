Step 2.5: Encoding and serialization
====================================

[Rust] has the well-known [serde] crate which provides the common approach and toolset for serialization and deserialization of data structures.

The sweet part is that [serde] __does not rely on a runtime reflection__ mechanism (but uses [Rust]'s trait system), which __eliminates most runtime costs__ and in most cases __makes serialization as performant as handwritten serializer for a particular case__, yet __remains ergonomic due to [automatic deriving][1]__.

```rust
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
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

[serde] represents a kind of serialization frontend, which can be backed by any format you desire. There are already [implemented backends][2] for most used formats, and you're free to [implement backend for your own format][3] if it's not implemented yet. 

Read through [official serde guide][0] carefully to understand its architecture, concepts, [usage][4] and features (like [zero-copy deserialization][5]).




## Task

Write a program which deserializes the [following JSON](request.json) into a `struct Request { .. }` and prints out its serialization in a YAML and TOML formats.





[Rust]: https://www.rust-lang.org
[serde]: https://crates.io/crates/serde

[0]: https://serde.rs
[1]: https://serde.rs/derive.html
[2]: https://serde.rs/index.html#data-formats
[3]: https://serde.rs/data-format.html
[4]: https://serde.rs/examples.html
[5]: https://serde.rs/lifetimes.html#understanding-deserializer-lifetimes
