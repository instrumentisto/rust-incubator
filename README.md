Rust Incubator
==============

> It wasn’t always so clear, but the Rust programming language is fundamentally about _empowerment_: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.
_<div align="right">Rust Book's Foreword</div>_

This project represents a hard-way step-by-step [Rust] learning course from language basics to a capability of web backend development.




## Prerequisites


### Toolchain

- [rustup] for installing [Rust] toolchain and keeping it up-to-date.
- [CLion]/[IntelliJ IDEA] + [IntelliJ Rust] + [Toml][IntelliJ Toml] plugins as development environment (or any other on your choice).


### Bookshelf

- [Rust Book] teaches and explains [Rust] basics.
- [Rust By Example] teaches you [Rust] basics using editable examples.
- [Rust Reference] is not a formal spec, but is more detailed and comprehensive than the [Rust Book].
- [Cheats.rs] and [Rust SVG Cheatsheet] for quick reference.
- [Rust Edition Guide] for considering improvements of [Rust 2018].
- [Rust std lib] documentation.
- [Cargo Book] is a guide to [Cargo], [Rust]'s build tool and dependency manager.
- [Rustdoc Book] is a guide to `rustdoc` documentation tool.
- [Rust Cookbook] is a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the [Rust] ecosystem.
- [Rust Design Patterns] is an open source repository of [Rust] design patterns and idioms.
- [Rust FAQ] answers common question about [Rust].
- [Rust Playground] allows to share and check runnable [Rust] code snippets online.
- [Awesome Rust] is a curated list of [Rust] code and resources.
- [This Week in Rust] represents handpicked and subscribable [Rust] weekly updates.
- [Baby Steps] blog of [Nicholas Matsakis](https://github.com/nikomatsakis) shares useful [Rust] patterns, ideas and design decisions.




## Steps


### Before you start

1. [Create][1] a new [GitHub repository] for yourself using this one [as template][2].
2. [Invite as a collaborator][3] of your repository the person you want to review your lessons (lead).


### Schedule

Each step must be performed as a separate [PR (pull request)][PR] with an appropriate name and checked here in README's schedule after completion.

Each step has estimated time for being completed. If any deeper investigation on step's theme is required for you, then it's on your own. 

Do not hesitate to ask your lead with questions, however you won't receive a concrete answer, but rather a direction for investigation. Lead is the one who asks questions about everything here and demands a concrete answers.

TODO: use rustfmt and Clippy.

- [ ] [0. Become familiar with Rust basics][Step 0] (3 days)
- [ ] 1. Concepts
    - [ ] [1.1. Default values, cloning and copying][Step 1.1] (1 day)
    - [ ] [1.2. Boxing and pinning][Step 1.2] (1 day)
    - [ ] [1.3. Shared ownership and interior mutability][Step 1.3] (1 day)
    - [ ] 1.4. Clone-on-write
    - [ ] 1.5. Conversions and dereferencing
    - [ ] 1.6. Static and dynamic dispatch
    - [ ] 1.7. `Sized` and `?Sized` types
    - [ ] 1.8. Thread safety
- [ ] 2. Idioms
    - [ ] 2.1. Rich types ensure correctness
    - [ ] 2.2. Swapping values with `mem::replace`
    - [ ] 2.3. Trait bounds on impls, not types
    - [ ] 2.4. Abstract type in, concrete type out
- [ ] 3. Tools and libraries
    - [ ] 3.1. Testing and mocking
    - [ ] 3.2. Declarative and procedural macros
    - [ ] 3.3. Date and time
    - [ ] 3.4. Regular expressions and parsers
    - [ ] 3.5. Collections and iterators
    - [ ] 3.6. Serialization and deserialization
    - [ ] 3.7. Randomness and cryptography
    - [ ] 3.8. Logging
    - [ ] 3.9. Command-line arguments, environment variables and configs
    - [ ] 3.10. Multithreading and parallelism
    - [ ] 3.11. Async I/O, futures and actors
    - [ ] 3.12. Web frameworks, databases, connection pools and ORMs





[Step 0]: 0_basics
[Step 1.1]: 1_concepts/1_1_default_clone_copy
[Step 1.2]: 1_concepts/1_2_box_pin
[Step 1.3]: 1_concepts/1_3_rc_cell

[Step 1]: 1_key_concepts

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
[CLion]: https://www.jetbrains.com/clion
[GitHub repository]: https://help.github.com/articles/github-glossary/#repository
[IntelliJ IDEA]: https://www.jetbrains.com/idea
[IntelliJ Rust]: https://intellij-rust.github.io
[IntelliJ Toml]: https://plugins.jetbrains.com/plugin/8195-toml
[PR]: https://help.github.com/articles/github-glossary/#pull-request
[Rust]: https://www.rust-lang.org
[Rust 2018]: https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust Cookbook]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust Design Patterns]: https://github.com/rust-unofficial/patterns
[Rust Edition Guide]: https://rust-lang-nursery.github.io/edition-guide
[Rust FAQ]: https://www.rust-lang.org/faq.html
[Rust Playground]: https://play.rust-lang.org
[Rust Reference]: https://doc.rust-lang.org/reference
[Rust std lib]: https://doc.rust-lang.org/std
[Rust SVG Cheatsheet]: https://www.breakdown-notes.com/make/load/rust_cs_canvas/true
[Rustdoc Book]: https://doc.rust-lang.org/rustdoc
[rustup]: https://rustup.rs
[This Week in Rust]: https://this-week-in-rust.org

[1]: https://github.com/instrumentisto/rust-incubator/generate
[2]: https://help.github.com/en/articles/creating-a-repository-from-a-template
[3]: https://help.github.com/en/articles/inviting-collaborators-to-a-personal-repository
