Step 2: Idioms
==============

__Estimated time__: 2 days

These steps describe common idioms required for writing well-designed and idiomatic [Rust] code.

Before completing this step you should complete all its sub-steps. After doing them you should be able to answer the following questions:
- Why should I care about types and expressing things in types? How do types help to increase guarantees of a program being correct?
- What is essential for writing well-designed and ergonomic APIs in [Rust] and why?
- Why `mem::replace` exists and what purpose does it sole? When and why is it really helpful?




## Task

Design and implement a `VendingMachine` type, which behaves like a [vending machine][1]:
- `Product` should has a price and a name;
- `VendingMachine` should have a limited capacity of `Product`s;
- `VendingMachine` should be able to give a rest;
- `VendingMachine` should reject purchase if it cannot give a rest;
- `Coin`s could have only `1`, `2`, `5`, `10`, `20` and `50` nominals.

Make its usage API as convenient as you're capable to.





[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Vending_machine
