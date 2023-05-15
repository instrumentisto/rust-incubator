Step 2.5: Exhaustivity
======================

__Estimated time__: 1 day

Exhaustiveness checking in [pattern matching][1] is a very useful tool, allowing to spot certain bugs at compile-time by cheking whether all the combinations of some values where covered and considered in the source code. Applied correctly, it increases the [fearless refactoring][2] quality of a source code, eliminating possibilities for "forgot to change" bugs to subtly sneak into the codebase whenever it's changed.




## Enums

The most canonical and iconic example of exhaustiveness checking is using an enum in a `match` expression. The point here is to [omit][5] using [`_` (wildcard pattern)][4] or match-anything bindings, as such `match` expressions won't break in compile-time when something new is added.

For example, this is a very bad code:
```rust
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        _ => Permissions::All, // anybody else is administrator 
    }
}
```
If, for some reason, a new `Role::Guest` is added, with very high probability this code won't be changed accordingly, introducing a security bug, by granting `Permissions::All` to any guest. This mainly happens, because the code itself doesn't signal to us in any way that it should be reconsidered.

By leveraging exhaustivity, the code can be altered in the way it breaks at compile-time whener a new `Role` variant is added:
```rust
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        Role::Admin => Permissions::All, 
    }
}
```
```
error[E0004]: non-exhaustive patterns: `&Role::Guest` not covered
  --> src/lib.rs:16:11
   |
16 |     match role {
   |           ^^^^ pattern `&Role::Guest` not covered
   |
note: `Role` defined here
  --> src/lib.rs:2:5
   |
1  | enum Role {
   |      ----
2  |     Guest,
   |     ^^^^^ not covered
```




## Structs

While enums exhaustiveness is quite an obvious idea, due to extensive usage of `match` expressions in a regular code, the structs exhaustiveness, on the other hand, is not, while being as much useful. Exhaustivity for structs is achieved by using [destructuring][6] without [`..` syntax (multiple fields ignoring)][7].

For example, having the following code:
```rust
struct Address {
    country: Country,
    city: City,
    street: Street,
    zip: Zip,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.country)?;
        writeln!(f, "{}", self.city)?;
        writeln!(f, "{}", self.street)?;
        write!(f, "{}", self.zip)
    }
}
```
It's super easy to forget changing the `Display` implementation when we add a new `state` field.

So, altering the code with exhaustive destructuring allows us to omit such a subtle bug, by breaking in compile-time:
```rust
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            country,
            city,
            street,
            zip,
        } = self;
        writeln!(f, "{country}")?;
        writeln!(f, "{city}")?;
        writeln!(f, "{street}")?;
        write!(f, "{zip}")
    }
}
```
```
error[E0027]: pattern does not mention field `state`
  --> src/lib.rs:19:13
   |
19 |           let Self {
   |  _____________^
20 | |             country,
21 | |             city,
22 | |             street,
23 | |             zip,
24 | |         } = self;
   | |_________^ missing field `state`
   |
help: include the missing field in the pattern
   |
23 |             zip, state } = self;
   |                ~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
   |
23 |             zip, .. } = self;
   |    
```

Another real-world use-cases of maintaining invariants covering all struct fields via exhaustivity checks are illustrated in the following articles:
- [Ashley Mannix: How we organize a complex Rust codebase][8]




[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/book/ch18-00-patterns.html
[2]: https://news.ycombinator.com/item?id=27553775
[3]: https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#match-arms
[4]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern
[5]: https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_enum_match_arm
[6]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values
[7]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-remaining-parts-of-a-value-with-
[8]: https://blog.datalust.co/rust-at-datalust-how-we-organize-a-complex-rust-codebase#maintaininginvariantsthatcoverallstructfields
