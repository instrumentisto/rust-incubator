Step 1.6: `Deref`: dereferencing
================================

Except common references (`&T`) [Rust] has many smart-pointer types in its standard library ([`Box`], [`Rc`], [`Cow`], etc.). Beyond that [Rust] allows you to implement your own custom smart-pointer types. However, [Rust] is a statically and strongly typed language, so you can't just use one type when other is expected, an explicit conversion is required. This may introduce some ergonomics issues when you're dealing with pointers and references in your code, as quite often you do not require any knowledge of how data is stored, and everything you require is _just a reference to the data_. Here is the situation where [`Deref`] plays in.

[`Deref`] standard library trait __allows to automatically coerce from a custom type to a reference__ when dereferencing (operator `*v`) is used. The most common example of this is using [`Box<T>`][`Box`] where `&T` is expected.

For better [`Deref`] understanding read through:
- [Rust Book: 15.2. The Deref Trait Allows Access to the Data Through a Reference][1]
- [Official `std::ops::Deref` docs][`Deref`]




## Incorrect usage

The automatic coercion that [Rust] implements for [`Deref`] is a sweet honey pot which may lead you to misuse of this ability.

The common temptation is to use [`Deref`] in a combination with "new type" pattern (introduced in [Step 1.1]), so you can use your inner type via outer type without any explicit requirements. However, this is considered to be a bad practice, and [official `Deref` docs][`Deref`] clearly states:
>>>
__`Deref` should only be implemented for smart pointers__
>>>

The wider explanation of this bad practice is given in [this SO answer][2].




## Task

Implement inefficient and ridiculous custom smart-pointer type `File<T>` which hold its data in a unique file, derefs to `&T`, and removes the corresponding file when is dropped.





[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[Rust]: https://www.rust-lang.org

[Step 1.1]: ../1_1_type_safety

[1]: https://doc.rust-lang.org/book/second-edition/ch15-02-deref.html
[2]: https://stackoverflow.com/questions/45086595/is-it-considered-a-bad-practice-to-implement-deref-for-newtypes
