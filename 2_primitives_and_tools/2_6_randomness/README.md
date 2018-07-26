Step 2.6: Randomness
====================

For random values generation [Rust] has [rand] crate in its ecosystem which provides unified interface and different random values generator implementations which vary statistical quality and performance on your choice.

[rand] also has [an excellent documentation][1] which not only explains how to use crate primitives, but also makes intro to basics of random values generation problem and how it's solved in modern world. Read through it to understand what primitives you should choose for different situations: when performance is a goal, when cryptographical security and good statical quality is a goal, or what should you use for general purpose.

One of the most common cases when you need to deal with generating random values is generation of universally unique identifiers (such as [UUID]). Fortunately, [Rust] has [uuid] crate already, which implements all versions of [UUID] specification. Read through its [documentation][2] to become familiar with its usage.




## Task

Implement 3 functions:
1. `generate_password()` which generates random password of given length and symbols set;
2. `select_rand_val()` which retrieves random element of given slice;
3. `new_access_token()` which generates unique cryptographically secure random value in `a-zA-Z0-9` symbols set and has exactly `64` symbols.

Cover your implementations with tests. Try to implement some of [Diehard tests].





[Rust]: https://www.rust-lang.org
[rand]: https://crates.io/crates/rand
[uuid]: https://crates.io/crates/uuid
[UUID]: https://en.wikipedia.org/wiki/Universally_unique_identifier

[Diehard tests]: https://en.wikipedia.org/wiki/Diehard_tests

[1]: https://docs.rs/rand
[2]: https://docs.rs/uuid
