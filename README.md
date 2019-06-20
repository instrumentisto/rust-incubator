Rust Incubator
==============

>>>
It wasn’t always so clear, but the Rust programming language is fundamentally about _empowerment_: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.
_<div align="right">Rust Book's Foreword</div>_
>>>

This project represents a step-by-step [Rust] learning course from language basics to production use.




## Prerequisites


### Toolchain

- [rustup] for installing [Rust] toolchain and keeping it up-to-date.
- [IntelliJ IDEA] + [IntelliJ Rust] and [Toml][IntelliJ Toml] plugins as development environment.
- [rustfmt] for formatting [Rust] code.


### Bookshelf

- [Rust Book] teaches and explains [Rust] basics.
- [Rust Reference] is not a formal spec, but is more detailed and comprehensive than the [Rust Book].
- [Cheats.rs] and [Rust SVG Cheatsheet] for quick reference.
- [Rust Edition Guide] for considering improvements of [Rust 2018].
- [Rust Standard Library] documentation.
- [Rust Style Guide] is an official style of formatting [Rust] code.
- [Cargo Book] is a guide to [Cargo], [Rust]'s build tool and dependency manager.
- [Rustdoc Book] is a guide to `rustdoc` documentation tool.
- [Rust By Example] teaches you how to program in Rust using editable examples.
- [Rust Cookbook] is a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the [Rust] ecosystem.
- [Rust FAQ] answers common question about [Rust].
- [Rust Playground] allows to share and check runnable [Rust] code snippets online.
- [Awesome Rust] is a curated list of Rust code and resources.
- [Baby Steps] blog of [Nicholas Matsakis](https://github.com/nikomatsakis) shares useful [Rust] patterns and ideas.




## Steps

Each step must be performed as separate [MR (Merge Request)][MR] with correspondent name and checked as done after it's completed.

Do not hesitate to ask your lead with questions.

- [ ] [0. Become familiar with Rust basics][Step 0]
- [ ] [1. Key concepts][Step 1]
    - [ ] [1.1. Type safety][Step 1.1]
    - [ ] [1.2. `Rc`/`Weak`: reference counting][Step 1.2]
    - [ ] [1.3. `Cell`/`RefCell`: interior mutability][Step 1.3]
    - [ ] [1.4. `Cow`: clone-on-write][Step 1.4]
    - [ ] [1.5. `Default`: default values][Step 1.5]
    - [ ] [1.6. `Deref`: dereferencing][Step 1.6]
    - [ ] [1.7. Static and dynamic dispatch][Step 1.7]
    - [ ] [1.8. `Send`/`Sync`: thread safety][Step 1.8]
- 2. Primitives and tools
    - [ ] [2.1. Date and time][Step 2.1]
    - [ ] [2.2. Regular expressions][Step 2.2]
    - [ ] [2.3. Bitmasks][Step 2.3]
    - [ ] [2.4. Collections][Step 2.4]
    - [ ] [2.5. Encoding and serialization][Step 2.5]
    - [ ] [2.6. Randomness][Step 2.6]
    - [ ] [2.7. Cryptography][Step 2.7]
    - [ ] [2.8. Logging][Step 2.8]
    - [ ] [2.9. Command-line arguments][Step 2.9]
    - [ ] [2.10. Environment variables][Step 2.10]
    - [ ] [2.11. Configuration][Step 2.11]
    - [ ] [2.12. Threads, synchronization and parallelism][Step 2.12]
    - [ ] [2.13. Futures and async I/O][Step 2.13]
    - [ ] [2.14. Actors][Step 2.14]
    - [ ] [2.15. Databases and ORMs][Step 2.15]
- 3. Architecture
    - 3.1. Long-running application
    - 3.2. Clean architecture





[Step 0]: 0_basics
[Step 1]: 1_key_concepts
[Step 1.1]: 1_key_concepts/1_1_type_safety
[Step 1.2]: 1_key_concepts/1_2_reference_counting
[Step 1.3]: 1_key_concepts/1_3_interior_mutability
[Step 1.4]: 1_key_concepts/1_4_clone_on_write
[Step 1.5]: 1_key_concepts/1_5_default_values
[Step 1.6]: 1_key_concepts/1_6_dereferencing
[Step 1.7]: 1_key_concepts/1_7_static_and_dynamic_dispatch
[Step 1.8]: 1_key_concepts/1_8_thread_safety
[Step 2.1]: 2_primitives_and_tools/2_1_date_and_time
[Step 2.2]: 2_primitives_and_tools/2_2_regular_expressions
[Step 2.3]: 2_primitives_and_tools/2_3_bitmasks
[Step 2.4]: 2_primitives_and_tools/2_4_collections
[Step 2.5]: 2_primitives_and_tools/2_5_encoding_and_serialization
[Step 2.6]: 2_primitives_and_tools/2_6_randomness
[Step 2.7]: 2_primitives_and_tools/2_7_cryptography
[Step 2.8]: 2_primitives_and_tools/2_8_logging
[Step 2.9]: 2_primitives_and_tools/2_9_command_line_arguments
[Step 2.10]: 2_primitives_and_tools/2_10_environment_variables
[Step 2.11]: 2_primitives_and_tools/2_11_configuration
[Step 2.12]: 2_primitives_and_tools/2_12_threads_synchronization_and_parallelism
[Step 2.13]: 2_primitives_and_tools/2_13_futures_and_async_io
[Step 2.14]: 2_primitives_and_tools/2_14_actors
[Step 2.15]: 2_primitives_and_tools/2_15_databases_and_orms

[Awesome Rust]: https://github.com/rust-unofficial/awesome-rust
[Baby Steps]: http://smallcultfollowing.com/babysteps
[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Cheats.rs]: https://cheats.rs
[Make]: https://www.gnu.org/software/make
[MR]: https://docs.gitlab.com/ce/user/project/merge_requests
[IntelliJ IDEA]: https://www.jetbrains.com/idea
[IntelliJ Rust]: https://intellij-rust.github.io
[IntelliJ Toml]: https://plugins.jetbrains.com/plugin/8195-toml
[Rust]: https://www.rust-lang.org
[Rust 2018]: https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust Cookbook]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust Edition Guide]: https://rust-lang-nursery.github.io/edition-guide
[Rust FAQ]: https://www.rust-lang.org/faq.html
[Rust Playground]: https://play.rust-lang.org
[Rust Reference]: https://doc.rust-lang.org/reference
[Rust Standard Library]: https://doc.rust-lang.org/std
[Rust Style Guide]: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md
[Rust SVG Cheatsheet]: https://www.breakdown-notes.com/make/load/rust_cs_canvas/true
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[Rustdoc Book]: https://doc.rust-lang.org/rustdoc
[rustup]: https://rustup.rs
