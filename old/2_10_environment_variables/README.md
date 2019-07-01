Step 2.10: Environment variables
================================

[Rust] provides common primitives in [`std::env`] for working with environment variables as strings. The basic example of its usage is described in [this chapter of Rust Book][1].

However, most of the time you want to operate not just with raw strings but typed data. Here is where [envy] crate plays in. It represents a backend for [serde], which allows you to use [serde attributes] for setting up deserialization in the way you want. Read through [envy docs] for its usage details.

Finally, [dotenv] crate should be mentioned. It sets environment variables based on `.env` file contents, which is widely used convention to simplify environment configuration and to not declare all required environment variables all the time you run program.




## Task

Write a simple program which lookups for 3 environment variables (`ENV_VAR_ONE`, `ENV_VAR_TWO`, `ENV_VAR_THREE`) and for each variable prints:
- `<absent>` if variable is not defined;
- `<empty>` if variable is defined but is empty string;
- value of variable in other cases.

Do it in both ways: by using only [`std::env`] and by using [envy] crate.

Use [dotenv] crate to set variables for testing.





[`std::env`]: https://doc.rust-lang.org/std/env/index.html
[Rust]: https://www.rust-lang.org
[dotenv]: https://crates.io/crates/dotenv
[envy]: https://crates.io/crates/envy
[envy docs]: https://docs.rs/envy
[serde]: https://serde.rs
[serde attributes]: https://serde.rs/attributes.html#field-attributes

[1]: https://doc.rust-lang.org/book/second-edition/ch12-05-working-with-environment-variables.html
