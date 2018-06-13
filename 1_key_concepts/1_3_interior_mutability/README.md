Step 1.3: `Cell`/`RefCell`: interior mutability
===============================================

[Rust] memory safety is based on the following rule (known as "borrowing rules"):

>>>
Given an object `T`, it is only possible to have one of the following:
- Having several immutable references (`&T`) to the object (also known as __aliasing__).
- Having one mutable reference (`&mut T`) to the object (also known as __mutability__).
>>>

However, there are situations where this rule is not flexible enough and it's required to have multiple references to an object and yet mutate it. [`Cell`] and [`RefCell`] __encapsulate mutability inside__ (thus called "interior mutability") __and provide interface which can be used through common shared references__ (`&T`).

These containers __allow to overcome [Rust] borrowing rules and track borrows at runtime__ (so called "dynamic borrowing"), which, obviously, leads to less safe code as compile-time errors become runtime panics. That's why one should __use [`Cell`]/[`RefCell`] wisely and only as last resort__.

For better understanding [`Cell`]/[`RefCell`] purpose, design and use cases read through:
- [Official `std:cell` docs][`std::cell`]
- [Rust Book: 15.5. RefCell and the Interior Mutability Pattern][1]




## Task

Write a simple `Stack<T>` collection which represents a trivial [stack] of given size `n` and can be mutated through multiple shared references (`&Stack<T>`).

Cover your implementation with tests.





[`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[`std::cell`]: https://doc.rust-lang.org/std/cell
[Rust]: https://www.rust-lang.org
[stack]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)

[1]: https://doc.rust-lang.org/book/second-edition/ch15-05-interior-mutability.html
