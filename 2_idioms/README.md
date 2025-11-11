Step 2: Idioms
==============

__Estimated time__: 2 days

These steps describe common idioms required for writing well-designed and idiomatic [Rust] code.

> ❗️Before completing this step you should complete all its sub-steps.

After doing them you should be able to answer the following questions:
- Why should I care about types and expressing things in types? How do types help to increase guarantees of a program being correct?
- What is essential for writing well-designed and ergonomic APIs in [Rust] and why?
- Why `mem::replace` exists and what purpose does it solve? When and why is it really helpful?
- How input type polymorphism is usually organized in [Rust] APIs? What cost does it have?
- Which ways and tools do exist for future-proofing source code in [Rust]?## More reading




## More reading

- [Matthias Endler: Idiomatic Rust: Patterns for Defensive Programming in Rust][11]
- [Rust Design Patterns]
- [Learning Material for Idiomatic Rust]




## Task

Design and implement a `VendingMachine` type, which behaves like a [vending machine][1]:
- `Product` should have a price and a name;
- `VendingMachine` should have a limited capacity of `Product`s;
- `VendingMachine` should be able to give change;
- `VendingMachine` should reject purchase if it cannot give change;
- `Coin` nominal values could only be `1`, `2`, `5`, `10`, `20` and `50`.

Make its usage API as convenient as you're capable to.




[Learning Material for Idiomatic Rust]: https://corrode.dev/blog/idiomatic-rust-resources
[Rust]: https://www.rust-lang.org
[Rust Design Patterns]: https://rust-unofficial.github.io/patterns

[1]: https://en.wikipedia.org/wiki/Vending_machine
[11]: https://corrode.dev/blog/defensive-programming
