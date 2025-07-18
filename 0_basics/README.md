Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

Read through [the Rust Book][Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, the memory model, and the type and module systems.

Polish your familiarity by completing [Rust By Example] and [Rustlings][rustlings].

Read through [the Cargo Book][Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, you should be able to answer (and understand why) the following questions:
- What memory model does [Rust] have? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
- What runtime does [Rust] have? Does it use a GC (garbage collector)?
- What does static typing mean? What is a benefit of using it?
- What are generics and parametric polymorphism? Which problems do they solve?
- What are traits? How are they used? How do they compare to interfaces? What are auto traits and blanket impls? What is a marker trait?
- What are static and dynamic dispatch? Which should you use, and when?
- What is a crate and what is a module in [Rust]? How do they differ? How are they used?
- What are move semantics? What are borrowing rules? What is the benefit of using them?
- What is immutability? What is the benefit of using it?
- What is cloning? What is copying? How do they compare?
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
- What is an iterator? What is a collection? How do they differ? How are they used?
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macros?
- How is code tested in [Rust]? Where should you put tests and why?
- Why does [Rust] have `&str` and `String` types? How do they differ? When should you use them?
- What are lifetimes? Which problems do they solve? Which benefits do they give?
- Is [Rust] an OOP language? Is it possible to use SOLID/GRASP? Does it have inheritance?

_Additional_ articles, which may help to understand the above topic better:
- [George He: Thinking in Rust: Ownership, Access, and Memory Safety][19]
- [Chris Morgan: Rust ownership, the hard way][1]
- [Adolfo Ochagavía: You are holding it wrong][12]
- [Vikram Fugro: Beyond Pointers: How Rust outshines C++ with its Borrow Checker][15]
- [Sabrina Jewson: Why the “Null” Lifetime Does Not Exist][16]
- [HashRust: A guide to closures in Rust][13]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Bradford Hovinen: Demystifying trait generics in Rust][14]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [Lukasz Uszko: Safe and Secure Coding in Rust: A Comparative Analysis of Rust and C/C++][18]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][10]
- [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust][11]
- [Clayton Ramsey: I built a garbage collector for a language that doesn't need one][17]




[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[rustlings]: https://rustlings.cool

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[11]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[12]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[13]: https://hashrust.com/blog/a-guide-to-closures-in-rust
[14]: https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust
[15]: https://dev.to/vikram2784/beyond-pointers-how-rust-outshines-c-with-its-borrow-checker-1mad
[16]: https://sabrinajewson.org/blog/null-lifetime
[17]: https://claytonwramsey.github.io/2023/08/14/dumpster.html
[18]: https://luk6xff.github.io/other/safe_secure_rust_book/intro/index.html
[19]: https://cocoindex.io/blogs/rust-ownership-access
