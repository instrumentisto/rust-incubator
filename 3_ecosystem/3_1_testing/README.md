Step 3.1: Testing and mocking
=============================

__Estimated time__: 1 day

[Rust] testing ecosystem [hasn't grown quite well yet][1], but provides some interesting libraries and solutions.




## Built-in testing capabilities

[Rust] provides quite good built-in testing capabilities, which are very well described in the following articles:
- [Rust Book: 11. Writing Automated Tests][2]
- [Rust By Example: 21. Testing][3]
- [Rust By Example: 12.3. Tests][4]




## BDD style

[BDD (behavior-driven development)][BDD] testing style implies that _test cases represent a program specification_, while _tests themselves prove the specification correctness_.

While [Rust] ecosystem has [some BDD testing style crates][11] (the most mature one is [cucumber_rust] crate), it's not a requirement to use them to follow [BDD] style (as they may be too complex for some trivial cases). There is nothing which prevents you from following [BDD] style in usual [Rust] tests. So, instead of:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash() {
        let h = hash("some_string");
        
        assert_eq!(h.len(), 64);
        assert!(!h.contains("z"));
    }
}
```
You're always free to write it more meaningfully:
```rust
#[cfg(test)]
mod hash_spec {
    use super::*;
    
    #[test]
    fn has_64_symbols_len() {
        assert_eq!(hash("some_string").len(), 64);
    }
    
    #[test]
    fn contains_hex_chars_only() {
        assert!(!hash("some_string").contains("z"));
    }
}
```
This makes tests more granular (so more meaningful test failures) and testing intentions become more understandable for reader.




## Mocking

Unfortunately, for [mocking][41] [Rust] ecosystem does not provide mature solutions. However, still there are [some crates][43] out [there][42].

The most interested one is [mockiato] crate at the moment, as is quite ergonomic in use and supports stable [Rust].

Additionally, [mockito] crate should be mentioned as a quite useful one for HTTP testing.

For better overview and familiarity with [mocking][41] in [Rust], read through the following articles:
- [Alan Somers: Rust Mock Shootout!][43]
- [Official `mockiato` crate docs][mockiato]
- [Official `mockito` crate docs][mockito]




## Property testing

[Property testing][21] is another testing paradigm for considering. In a nutshell, it can be explained in the following way:

> _Property testing_ is a system of testing code by checking that certain properties of its output or behaviour are fulfilled for all inputs. These inputs are generated automatically, and, critically, when a failing input is found, the input is automatically reduced to a _minimal_ test case.

[Rust] ecosystem has quite good [proptest] and [quickcheck] crates, which provide tools and primitives for [property testing][21].

For better understanding and familiarity with [property testing][21] in [Rust], read through the following articles:
- [`proptest` crate description][proptest]
- [`quickcheck` crate description][quickcheck]
- [Proptest Book][22]




## Fuzzing

[Fuzzing][31] is another testing technique, which involves providing invalid, unexpected, or random data as inputs to a computer program. It [really helps][32] to spot program crashes and memory leaks in edge cases.

[Rust] ecosystem has [several tools][33] for [fuzzing][31] at the moment. Most known are:
- [cargo-fuzz] is a command-line wrapper for using [libFuzzer].
- [afl.rs] allows to run [AFL (american fuzzy lop)][AFL] on code written in [Rust].
- [honggfuzz] is a security oriented fuzzer with powerful analysis options, which supports evolutionary, feedback-driven fuzzing based on code coverage (software- and hardware-based).

For better understanding and familiarity with [fuzzing][31] in [Rust], read through the following articles:
- [Rust Fuzz Book][34]
- [Official `cargo-fuzz` crate docs][cargo-fuzz]
- [Official `honggfuzz` crate docs][honggfuzz]




## Task

For the implementation of a small [guessing game][51] in [this step's crate](src/main.rs) provide all possible tests you're able to write.





[AFL]: http://lcamtuf.coredump.cx/afl
[afl.rs]: https://github.com/rust-fuzz/afl.rs
[BDD]: https://en.wikipedia.org/wiki/Behavior-driven_development
[cargo-fuzz]: https://docs.rs/cargo-fuzz
[cucumber_rust]: https://github.com/bbqsrc/cucumber-rust
[honggfuzz]: https://docs.rs/honggfuzz
[libFuzzer]: https://llvm.org/docs/LibFuzzer.html
[mockiato]: https://docs.rs/mockiato
[mockito]: https://docs.rs/mockito
[proptest]: https://crates.io/crates/proptest
[quickcheck]: https://crates.io/crates/quickcheck
[Rust]: https://www.rust-lang.org

[1]: https://github.com/rust-unofficial/awesome-rust#testing
[2]: https://doc.rust-lang.org/book/ch11-00-testing.html
[3]: https://doc.rust-lang.org/rust-by-example/testing.html
[4]: https://doc.rust-lang.org/rust-by-example/cargo/test.html
[11]: https://crates.io/search?q=bdd
[21]: https://en.wikipedia.org/wiki/Property_testing
[22]: https://altsysrq.github.io/proptest-book/intro.html
[31]: https://en.wikipedia.org/wiki/Fuzzing
[32]: https://github.com/rust-fuzz/trophy-case
[33]: https://crates.io/search?q=fuzzing
[34]: https://rust-fuzz.github.io/book/cargo-fuzz.html
[41]: https://en.wikipedia.org/wiki/Mock_object
[42]: https://crates.io/search?q=mocking
[43]: https://asomers.github.io/mock_shootout
[51]: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
