Step 1: Key concepts
====================

These steps describe common concepts and idioms that you should know to program in [Rust] in a convenient and idiomatic way.

Before completing this step you should complete all its sub-steps. After doing them you should be able to answer the following questions:
- How do I recognize that data is allocated at the heap rather than at the stack? When data should be allocated at the heap?
- What is copying and cloning data in [Rust]? What's the difference? When and why should I use them?
- How can a single piece of data be owned by multiple parts of program? When and why is this commonly required?
- How borrowing rules may be violated? In what price? When and why is this commonly required?
- How to deal with owned and borrowed data simultaneously? When and why is this commonly required?
- How to share values between threads? What is `Send` and `Sync` markers? Why are they required, when should be used?
- How do static and dynamic dispatches differ? Why do they exist? When and why should I choose between them?

The following articles may help you to sum up your experience:
- [Wrapper Types in Rust: Choosing Your Guarantees][1]
- [Rust, Builder Pattern, Trait Objects, `Box<T>` and `Rc<T>`][2]
- [Rust's Built-in Traits, the When, How & Why][3]

Notify your lead in related MR when you're done, and he will exam what you have learned.





[Rust]: https://www.rust-lang.org

[1]: https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees
[2]: https://abronan.com/rust-trait-objects-box-and-rc
[3]: https://llogiq.github.io/2015/07/30/traits.html
