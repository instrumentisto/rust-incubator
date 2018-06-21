Step 1.7: Static and dynamic dispatch
=====================================

[Static][1] and [dynamic][2] dispatches are important concepts to understand how your code is compiled and works in runtime, and how you can implement certain day-to-day common coding problems (related to polymorphism).

__[Static dispatch][1]__ (also called "early binding") __happens only at compile time__. The compiler generates separate code for each concrete type that is used. In [Rust] static dispatch is a __default way for polymorphism__ and is introduced simply by generics (parametric polymorphism): `MyType<T, S, F>`.

__[Dynamic dispatch][2]__ (sometimes called "late binding") __happens at runtime__. The concrete used type is erased at compile time, so compiler doesn't know it, therefore generates `vtable` which dispatches call at runtime, but __comes in a performance penalty__. In [Rust] dynamic dispatch is introduced via [trait objects][3]: `&MyTrait`, `Box<MyTrait>`.

You have to use [dynamic dispatch][2] in situations where [type erasure][4] is required. If the problem can be solved with a [static dispatch][1] then you'd better to do so to avoid performance penalties. The most common example when you cannot use [static dispatch][1] and have to go with [dynamic dispatch][2] are _heterogeneous_ collections (where each item is potentially a different concrete type, but each one implements `MyTrait`).

For better [static][1]/[dynamic][2] understanding and use cases, read through the following articles:
- [Abstraction without overhead: traits in Rust][11]
- [Traits and Trait Objects in Rust][12]
- [Rust Book: 17.2. Using Trait Objects that Allow for Values of Different Types][3]




## Task

Implement the simple `Queue<T>` collection of fixed size `n` that can be used both as a single type collection (without performance penalty for dynamic dispatch) and as a heterogeneous collection (which can hold dierent concrete types).

Cover you implementation with tests.





[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Static_dispatch
[2]: https://en.wikipedia.org/wiki/Dynamic_dispatch
[3]: https://doc.rust-lang.org/book/second-edition/ch17-02-trait-objects.html
[4]: https://en.wikipedia.org/wiki/Type_erasure
[11]: https://blog.rust-lang.org/2015/05/11/traits.html
[12]: https://joshleeb.com/posts/rust-traits-and-trait-objects
