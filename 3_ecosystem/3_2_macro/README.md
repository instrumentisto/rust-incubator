Step 3.2: Declarative and procedural macros
===========================================

__Estimated time__: 1 day

[Rust] provides strong and convenient built-in capabilities for code generation in a form of [macros][1].

> The term macro refers to a family of features in Rust: _declarative_ macros with `macro_rules!` and three kinds of _procedural_ macros:
> - Custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums
> - Attribute-like macros that define custom attributes usable on any item
> - Function-like macros that look like function calls but operate on the tokens specified as their argument




## Declarative macros

Declarative macros represent the most primitive form of macros in [Rust]. They are quite limited in their capabilities and their syntax (which represents a [DSL]-based `match` expression) may become quite cumbersome in complex cases.

They are called _declarative_, because macro implementation represents a declaration of code transforming rules (you're declaring how your code will be transformed):
```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = vec![1, 2, 3];
```
The good part about declarative macros is that they are [hygienic][11].

Code generation purpose is not the only one declarative macros are used for. Quite often they are used for building abstractions and APIs too, because they all to implement much more ergonomic features than regular functions do: named arguments, [variadics][17], etc.

For better understanding declarative macros design, concepts, usage and features, read through the following articles:
- [Rust Book: 19.6. Macros: Declarative Macros with `macro_rules!` for General Metaprogramming][13]
- [Rust By Example: 16. macro_rules!][14]
- [The Little Book of Rust Macros][15]
- [Rust Reference: 3.1. Macros By Example][16]




## Procedural macros

Procedural macros represent much more powerful code generation tool. They are called _procedural_, because macro implementation represents a regular [Rust] code, which works directly with [AST] of transformed code (you're writing procedures which transform your code). Procedural macro __requires a separate `proc-macro = true` crate__ to be implemented in.

Procedural macros are [unhygienic][11], so implementing one you need to be careful to ensure that macro works in [as many contexts as possible][22].

There are three kinds of procedural macros in [Rust] at the moment:

- [`proc_macro` function-like macros][27], which usage looks like regular declarative macros usage, but they accept arbitrary tokens on input (while declarative ones don't), and are more powerful in general (can contain complex logic for generating simple code):
    ```rust
    #[proc_macro]
    pub fn make_answer(_: TokenStream) -> TokenStream {
        "fn answer() -> u32 { 42 }".parse().unwrap()
    }
    ```
    ```rust
    make_answer!();
    ```

- [`proc_macro_attribute` attribute macros][28], which allow to create custom [Rust attributes][25]:
    ```rust
    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
       // code...
    }
    ```
    ```rust
    #[route(GET, "/")]
    fn index() {}
    ```

- [`proc_macro_derive` derive macros][29], which allow to provide custom implementations for `#[derive(Trait)]` attribute:
    ```rust
    #[proc_macro_derive(AnswerFn)]
    pub fn derive_answer_fn(_: TokenStream) -> TokenStream {
        "impl Struct{ fn answer() -> u32 { 42 } }".parse().unwrap()
    }
    ```
    ```rust
    #[derive(AnswerFn)]
    struct Struct;
    ```
    Idiomatically, `proc_macro_derive` should be used for _deriving trait implementations only_. For arbitrary functions generation it's better to go with `proc_macro_attribute`.

[Rust] ecosystem has some well-know crates, which almost always are used for procedural macros implementation:
- [syn] crate represents implementation of [Rust]'s [AST].
- [quote] crate provides quasi-quoting, which allows to turn [Rust] syntax tree data structures into tokens of source code in an ergonomic and readable way.
- [proc-macro2] crate provides unified [`proc_macro`] API across all [Rust] compiler versions and makes procedural macros unit testable.

Additionally, [darling] crate should be mentioned, which makes declarative attribute parsing more straight-forward and ergonomic.

For better understanding procedural macros design, concepts, usage and features, read through the following articles:
- [Rust Book: 19.6. Macros: Procedural Macros for Generating Code from Attributes][23]
- [Rust Reference: 3.2. Procedural Macros][26]
- [Official `syn` crate docs][syn]
- [Official `quote` crate docs][quote]
- [Official `proc-macro2` crate docs][proc-macro2]




## Task

Implement a `btreemap!` macro, which allows to create [`BTreeMap`] in an ergonomic and declarative way (similarly to `vec!`).

Provide two implementations: one via declarative macro and other one via procedural macro.





[AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[darling]: https://docs.rs/darling
[DSL]: https://en.wikipedia.org/wiki/Domain-specific_language
[`proc_macro`]: https://doc.rust-lang.org/proc_macro
[proc-macro2]: https://docs.rs/proc-macro2
[Rust]: https://www.rust-lang.org
[syn]: https://docs.rs/syn
[quote]: https://docs.rs/quote

[1]: https://en.wikipedia.org/wiki/Macro_(computer_science)
[11]: https://en.wikipedia.org/wiki/Hygienic_macro
[13]: https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming
[14]: https://doc.rust-lang.org/rust-by-example/macros.html
[15]: https://danielkeep.github.io/tlborm/book/README.html
[16]: https://doc.rust-lang.org/reference/macros-by-example.html
[17]: https://doc.rust-lang.org/rust-by-example/macros/variadics.html
[22]: https://rust-lang-nursery.github.io/api-guidelines/macros.html#item-macros-work-anywhere-that-items-are-allowed-c-anywhere
[23]: https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes
[25]: https://doc.rust-lang.org/reference/attributes.html
[26]: https://doc.rust-lang.org/reference/procedural-macros.html
[27]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
[28]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
[29]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
