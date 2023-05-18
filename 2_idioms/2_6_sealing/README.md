Step 2.6: Sealing
=================

__Estimated time__: 1 day

Sealing, in programming, usually means that some API (mostly public) cannot be inherited, extended or implemented outside its definition place. For example, a [sealed class or interface in Kotlin][1] cannot be inherited or implemented outside the library where it's defined. In [Rust], this idiom may be applied to [traits][2]. 




## Traits

__Sealed trait__ is a __publicly accessible__ trait, which __cannot be implemented outside its definition place__ (__module or crate__, depending on the visibility of this trait).

> ```rust
> /// This trait is sealed and cannot be implemented for types outside this crate.
> pub trait TheTrait: private::Sealed {
>     // Zero or more methods that the user is allowed to call.
>     fn ...();
>
>     // Zero or more private methods, not allowed for user to call.
>     #[doc(hidden)]
>     fn ...();
> }
>
> // Implement for some types.
> impl TheTrait for usize {
>     /* ... */
> }
>
> mod private {
>     pub trait Sealed {}
>
>     // Implement for those same types, but no others.
>     impl Sealed for usize {}
> }
> ```
> The empty private `Sealed` supertrait cannot be named by downstream crates, so we are guaranteed that implementations of `Sealed` (and therefore `TheTrait`) only exist in the current crate.

This is the most common way to seal a trait. The boilerplate could be completely cut off by using a [`sealed`] crate, providing a convenient macro to generate the one:
```rust
use sealed::sealed;

#[sealed]
pub trait TheTrait {}

#[sealed]
impl TheTrait for usize {}
```

However, there are alternative ways to seal a trait [via its method signature][5], or even [seal it partially][6].

The main purpose of sealing a trait is, of course, [future-proofing][7] of [API]s.

> We are free to add methods to `TheTrait` in a non-breaking release even though that would ordinarily be a breaking change for traits that are not sealed. Also we are free to change the signature of methods that are not publicly documented.

It's important to note that __trait sealing fully relies on__ tricking over visibility rules (__using a public [supertrait][8]__ or type, which __name is not publicly exported__), and so, has no impact on the type system semantics (a sealed public trait is just a regular public trait from the type system perspective). In theory, sealing a trait should affect its [coherence][9], by [relaxing its strictness for the use-cases which can never happen with a sealed trait][10]. However, that would require a special support by compiler, which seems [not gonna happen in the near future][11].

For better understanding traits sealing, its design and use-cases, read through the following articles:
- [Rust API Guidelines: 10. Future proofing: Sealed traits protect against downstream implementations (C-SEALED)][3]
- [Predrag Gruevski: A definitive guide to sealed traits in Rust][4]
- [Official `sealed` crate docs][`sealed`]




[`sealed`]: https://docs.rs/sealed
[API]: https://en.wikipedia.org/wiki/API
[Rust]: https://www.rust-lang.org

[1]: https://kotlinlang.org/docs/sealed-classes.html
[2]: https://doc.rust-lang.org/book/ch10-02-traits.html
[3]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
[4]: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust
[5]: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust#sealing-traits-via-method-signatures
[6]: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust#partially-sealed-traits
[7]: https://en.wikipedia.org/wiki/Future-proof
[8]: https://doc.rust-lang.org/reference/items/traits.html#supertraits
[9]: https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence
[10]: https://stackoverflow.com/questions/50012745/is-there-a-way-to-tell-the-compiler-that-nobody-will-implement-a-trait-for-a-ref
[11]: https://internals.rust-lang.org/t/sealed-traits/16797
