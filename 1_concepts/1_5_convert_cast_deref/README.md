Step 1.5: Conversions, casting and dereferencing
================================================

__Estimated time__: 1 day

As [Rust] is a [strongly typed][1] language, all type conversions must be performed explicitly in the code. As [Rust] has a rich type system (programming logic and semantics are mostly expressed in types rather than in values), type conversions are inevitable in almost every single line of code. Fortunately, [Rust] offers [well-designed type conversion capabilities][`std::convert`], which are quite ergonomic, intuitive and are pleasant to use.




## Value-to-value conversions

Value-to-value conversion in [Rust] is done with [`From`] and [`Into`] mirrored traits (implementing one automatically implements another one). These traits provide __non-fallible conversion__.

If your conversion may fail, then you should use [`TryFrom`]/[`TryInto`] analogues, which __allow failing in a controlled way__.

```rust
let num: u32 = 5;
let big_num: u64 = num.into();
let small_num: u16 = big_num.try_into().expect("Value is too big");
```

Note, that __all these traits consume ownership__ of a passed value. However, they [can be implemented for references too][2] if you're treating a reference as a value.

For better understanding [`From`]/[`Into`] and [`TryFrom`]/[`TryInto`] purpose, design, limitations and use cases read through:
- [Rust By Example: 6.1. From and Into][8]
- [Official `From` docs][`From`]
- [Official `Into` docs][`Into`]
- [Official `TryFrom` docs][`TryFrom`]
- [Official `TryInto` docs][`TryInto`]




## Reference-to-reference conversions

Quite often you don't want to consume ownership of a value for conversion, but rather to refer it as another type. In such case [`AsRef`]/[`AsMut`] should be used. They allow to do a __cheap non-fallible reference-to-reference conversion__.

```rust
let string: String = "some text".into();
let bytes: &[u8] = string.as_ref();
```

[`AsRef`]/[`AsMut`] are commonly implemented for smart pointers to allow referring a data behind it via regular [Rust] references.

For better understanding [`AsRef`]/[`AsMut`] purpose, design, limitations and use cases read through:
- [Official `AsRef` docs][`AsRef`]
- [Official `AsMut` docs][`AsMut`]
- [Ricardo Martins: Convenient and idiomatic conversions in Rust][10]




## Dereferencing

[`Deref`]/[`DerefMut`] standard library trait __allows to implicitly coerce from a custom type to a reference__ when dereferencing (operator `*v`) is used. The most common example of this is using [`Box<T>`][`Box`] where `&T` is expected.

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

let m = Box::new(String::from("Rust"));
hello(&m);
```

For better understanding [`Deref`] purpose, design, limitations and use cases read through:
- [Rust Book: 15.2. Treating Smart Pointers Like Regular References with the Deref Trait][3]
- [Official `Deref` docs][`Deref`]


### Incorrect usage

The implicit coercion that [Rust] implements for [`Deref`] is a sweet honey pot which may lead you to misuse of this feature.

The common temptation is to use [`Deref`] in a combination with [newtype pattern][4], so you can use your inner type via outer type without any explicit requirements. However, this is considered to be a bad practice, and [official `Deref` docs][`Deref`] clearly states:

> __`Deref` should only be implemented for smart pointers.__

The wider explanation of this bad practice is given in [this SO answer][5] and [`Deref` polymorphism anti-pattern][6] description.




## Casting

For casting between types the [`as` keyword][`as`] is used in [Rust].

```rust
fn average(values: &[f64]) -> f64 {
    let sum: f64 = sum(values);
    let size: f64 = len(values) as f64;
    sum / size
}
```

However, it supports only a [small, fixed set of transformations][7], and __is not idiomatic to use when other conversion possibilities are available__ (like [`From`], [`TryFrom`], [`AsRef`]).

See also:
- [Rust By Example: 5.1. Casting][9]
- [Rust Reference: 8.2.4. Type cast expressions][7]




## Task

Implement the following types:
1. `EmailString` - a type, which value can be only a valid email address string.
2. `Random<T>` - a smart pointer, which takes 3 values of the pointed-to type on creation and points to one of them randomly every time is used.

Provide conversion and `Deref` implementations for these types on your choice, to make their usage and interoperability with `std` types easy and ergonomic.





[`as`]: https://doc.rust-lang.org/std/keyword.as.html
[`AsMut`]: https://doc.rust-lang.org/std/convert/trait.AsMut.html
[`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
[Rust]: https://www.rust-lang.org
[`std::convert`]: https://doc.rust-lang.org/std/convert/index.html
[`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[`TryInto`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html

[1]: https://en.wikipedia.org/wiki/Strong_and_weak_typing
[2]: https://doc.rust-lang.org/std/string/struct.String.html#impl-From%3C%26%27_%20str%3E
[3]: https://doc.rust-lang.org/book/ch15-02-deref.html
[4]: https://github.com/rust-unofficial/patterns/blob/master/patterns/newtype.md
[5]: https://stackoverflow.com/questions/45086595/is-it-considered-a-bad-practice-to-implement-deref-for-newtypes
[6]: https://github.com/rust-unofficial/patterns/blob/master/anti_patterns/deref.md
[7]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
[8]: https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
[9]: https://doc.rust-lang.org/rust-by-example/types/cast.html
[10]: https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust
