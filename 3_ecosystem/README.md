Step 3: Ecosystem
=================

__Estimated time__: 5 days

These steps describe common crates and tools in [Rust] ecosystem required for web backend development.

Before completing this step you should complete all its sub-steps. After doing them you should be able to answer the following questions:
- What testing capabilities does [Rust] offer and when should I use them? Why should I follow [BDD] style?
- What are macros? How do they differ? What benefits does their usage give? When should I write one?
- How to work with date and time in [Rust]? How should I store time? How should I return it to other applications?
- How are regular expressions used in [Rust]? When are they not enough? How can I write a custom parser in [Rust]?
- How do iterator and collection compare and differ in [Rust]? What is the purpose of immutable collections? Why should I care about using concurrent collections?
- What should I use for serialization in [Rust]? Why this is good or bad?
- How can I generate randomness in [Rust]? Which guarantees of random generator should I choose and when?
- What should I use for password hashing in [Rust]? How can I encrypt a message with [Rust]? How should I compare secret values and why?
- How logging is organized in [Rust] ecosystem? Why should I care about structured logging?
- What should I use for building CLI interface in [Rust]? How can I organize a configuration for my application and why?
- Why multithreading is required for [Rust] programs and what problems does it solve? How threads concurrency differs with parallelism? How can I parallelize code in [Rust]?
- What is asynchronicity and what problems does it solve? How is it compared to threads concurrency? What is [Rust] solution for asynchronicity and why it has such design?
- What are actors? When are they useful?
- What should I use for HTTP server implementation in [Rust], when and why?
- How should I interact with databases in [Rust] application and why? How can I organize migrations for my project?


## Some usefull tools

- [cross-rs/cross](https://github.com/cross-rs/cross)
- [cargo-hack](https://github.com/taiki-e/cargo-hack)
- [Miri: unsafe core interpreter](https://github.com/rust-lang/miri)
- [cargo-outdated](https://crates.io/crates/cargo-outdated)
- [cargo-modules](https://github.com/regexident/cargo-modules)
- [cargo-make](https://github.com/sagiegurari/cargo-make)
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)


[BDD]: https://en.wikipedia.org/wiki/Behavior-driven_development
[Rust]: https://www.rust-lang.org
