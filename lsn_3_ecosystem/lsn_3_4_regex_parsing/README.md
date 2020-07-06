Step 3.4: Regular expressions and custom parsers
================================================

__Estimated time__: 1 day




## Regular expressions

To operate with [regular expressions][1] there is [regex] crate in [Rust] ecosystem.

Important to know, that in [Rust] __regular expression needs to be compiled before we can use it__. The compilation is not cheap. So, the following code introduces a performance problem:
```rust
fn is_email(email: &str) -> bool {
    let re = Regex::new(".+@.+").unwrap();  // compiles every time the function is called
    re.is_match(email)
}
```

To omit unnecessary performance penalty we should __compile regular expression once and reuse its compilation result__. This is easily achieved by using [lazy_static] crate both in global and/or local scopes:
```rust
lazy_static! {
    static ref REGEX_EMAIL: Regex = Regex::new(".+@.+").unwrap();
}  // compiles once on first use 

fn is_email(email: &str) -> bool {
    REGEX_EMAIL.is_match(email)
}
```

This may feel different with how [regular expressions][1] are used in other programming languages, because some of them implicitly cache compilation results and/or do not expose compilation API at all (like [PHP]). But if your background is a language like [Go] or [Java], this concept should be familiar to you.




## Custom parsers

If regular expressions are [not powerful enough][2] for your parsing problem, then you are ended up with writing your own parser. [Rust] ecosystem has [numerous][3] crates to help with that:
- [nom] is a [parser combinator][4] library. Nearly most performant among others. Especially good for parsing binary stuff (byte/bit-oriented).
- [pest] is a general purpose parser, which uses [PEG (parsing expression grammar)][5] as input and derives parser's code for it.
- [lalrpop] is a parser generator framework, which generates [LR(1) parser][6] code from custom grammar files.
- [combine] is an another [parser combinator][4] library, inspired by the [Haskell] library [Parsec].




## Task

Given the following [Rust `fmt` syntax grammar][7]:
```
format_spec := [[fill]align][sign]['#']['0'][width]['.' precision][type]
fill := character
align := '<' | '^' | '>'
sign := '+' | '-'
width := count
precision := count | '*'
type := identifier | '?' | ''
count := parameter | integer
parameter := argument '$'
```

Implement a parser to parse `sign`, `width` and `precision` from a given input (assumed to be a `format_spec`).

Provide implementations in two flavours: [regex]-based and via building a custom parser.

Prove your implementation correctness with tests.





[combine]: https://docs.rs/combine
[Go]: https://golang.org
[Haskell]: https://www.haskell.org
[Java]: https://www.java.com
[lalrpop]: https://docs.rs/lalrpop
[lazy_static]: https://docs.rs/lazy_static
[nom]: https://docs.rs/nom
[Parsec]: https://hackage.haskell.org/package/parsec
[PHP]: https://php.net
[pest]: https://docs.rs/pest
[regex]: https://docs.rs/regex
[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Regular_expression
[2]: https://stackoverflow.com/questions/1732348/regex-match-open-tags-except-xhtml-self-contained-tags
[3]: https://github.com/rust-unofficial/awesome-rust#parsing
[4]: https://en.wikipedia.org/wiki/Parser_combinator
[5]: https://en.wikipedia.org/wiki/Parsing_expression_grammar
[6]: https://en.wikipedia.org/wiki/Canonical_LR_parser
[7]: https://doc.rust-lang.org/std/fmt/index.html#syntax
[8]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
