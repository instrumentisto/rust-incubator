Step 2.2: Regular expressions
=============================

[Rust] has [regex] crate to operate with [regular expressions][1]. Read through its [documentation][regex docs] and become familiar with it concepts.




## Regex compilation

In [Rust] regular expression must be compiled before we can use it. The compilation is not cheap. So, the following code introduces a performance problem:
```rust
fn is_email(email: &str) -> bool {
    let re = Regex::new(".+@.+").unwrap();  // compiles every time the function is called
    re.is_match(email)
}
```

So, what do we need to omit unnecessary performance penalty?  
Simple: __compile once and reuse compilation result__.

This may be reached by using [lazy_static] crate both in global and/or local scopes:
```rust
lazy_static! {
    static ref REGEX_EMAIL: Regex = Regex::new(".+@.+").unwrap();
}  // compiles once on program startup 

fn is_email(email: &str) -> bool {
    REGEX_EMAIL.is_match(email)
}
```

This may feel different with how regular expressions are used in your previous programming languages, because some of them implicitly cache compilation results and/or do not expose compilation API at all (like PHP). But if you're backgrounded with languages like Go or Java, this concept should be familiar to you.




## Task

Implement a struct `User` with an `email` field, and its methods:
- `validate_and_set_email()` which accepts `&str`, validates it to be a valid email address, and assigns as user's email;
- `email_domain()` which parses domain part from user's email address with regular expression and returns it.

Prove your implementation correctness with tests.





[lazy_static]: https://crates.io/crates/lazy_static
[regex]: https://crates.io/crates/regex
[regex docs]: https://docs.rs/regex
[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Regular_expression
