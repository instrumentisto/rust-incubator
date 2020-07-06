Step 3.6: Serialization and deserialization
===========================================

__Estimated time__: 1 day

[Rust] ecosystem has the well-known [serde] crate, which provides common approach and toolset for serialization and deserialization.

The sweet part is that [serde] __does not rely on a runtime reflection__ mechanism and uses trait implementation for each type, so __eliminates most runtime costs__ and in most cases __makes serialization as performant as handwritten serializer for a particular case__, yet __remains ergonomic due to [automatic code deriving][1]__.

```rust
use serde::{Deserialize, Serialize};

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

[serde] by itself represents only an universal serialization frontend, which can be backed by actual implementation for any format. There are already [implemented backends for most used formats][2], and you're free to [implement backend for your own format][3] if it's not implemented yet. 

For better understanding and familiarity with [serde]'s design, concepts, usage, and features (like [zero-copy deserialization][5]), read through the following articles:
- [Official `serde` crate guide][0]
- [Official `serde` crate docs][serde]
- [Official `serde_json` crate docs][serde_json]




## Task

Write a program which deserializes the [following JSON](request.json) into a static `Request` type and prints out its serialization in a YAML and TOML formats. Consider to choose correct types for data representation.

Prove your implementation correctness with tests.





[Rust]: https://www.rust-lang.org
[serde]: https://docs.rs/serde
[serde_json]: https://docs.rs/serde_json

[0]: https://serde.rs
[1]: https://serde.rs/derive.html
[2]: https://serde.rs/index.html#data-formats
[3]: https://serde.rs/data-format.html
[4]: https://serde.rs/examples.html
[5]: https://serde.rs/lifetimes.html#understanding-deserializer-lifetimes
