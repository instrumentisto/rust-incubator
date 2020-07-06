Step 2.1: Rich types ensure correctness
=======================================

__Estimated time__: 1 day

[Rust] has a rich type system which allows to express our program primitives, entities, notions, logic and semantics mostly in types, rather than in data/values, which is known as a "programming with types" concept. The benefits of this are obvious: the more compiler knows about our problem - the more false programs it will decline. Or, rephrased: __the more we describe about the program in types - the more we reduce the probability for the program to be incorrect__.

"Programming with types" inevitably implies its own idioms and patterns. The most common are described below.




## Newtype

Consider the following example, which demonstrates a possible bug:
```rust
#[derive(Clone)]
struct Post {
    id: u64,
    user_id: u64,
    title: String,
    body: String,
}

fn repost(post: &Post, new_author_id: u64) -> Post {
    let mut new_post = post.clone();
    new_post.id = new_author_id;  // Oops!
    new_post
}
```
Here the problem occurs because our entities are expressed in values, so compiler makes no difference between `Post::id` and `Post::user_id` as they have the same type.

Let's express those entities in types:
```rust
mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(String);
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);
}

#[derive(Clone)]
struct Post {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

fn repost(post: &Post, new_author_id: user::Id) -> Post {
    let mut new_post = post.clone();
    new_post.id = new_author_id;  // Does not compile!
    new_post
}
```
Now, compiler is able to cut off this type of bugs _totally_ at compile time, and to be quite informative with errors:
```rust
error[E0308]: mismatched types
  --> src/main.rs:27:19
   |
27 |     new_post.id = new_author_id;
   |                   ^^^^^^^^^^^^^ expected struct `post::Id`, found struct `user::Id`
   |
   = note: expected type `post::Id`
              found type `user::Id`
```

This is what is called ["newtype pattern"][1]. [Newtypes][1] are a zero-cost abstraction - __there is no runtime overhead__. Additionally, you may __enforce desired invariants on values of the type__ (for example, `Email` type may allow only valid email address strings to be its values, and another good example is [uom] crate). Also, [newtype pattern][1] __makes code more understandable for developers__, as domain knowledge is reflected in types, so is described and documented more explicitly.

The downside of using [newtype pattern][1] is a necessity of writing _more boilerplate code_, because you should provide common traits implementations by yourself (like `Clone`, `Copy`, `From`/`Into`/`AsRef`/`AsMut`), as without them the type won't be ergonomic in use. However, most of them can be _derived automatically_ with `std` capabilities or third-party derive-crates (like [derive_more]), so the cost is acceptable in most cases.

For better understanding [newtype pattern][1], read through the following articles:
- [Rust Design Patterns: Newtype][1]
- [Rust By Example: 14.7. New Type Idiom][2]




## Typestates

[Newtype pattern][1] prevents us from invalid use of data. But what about behavior? Can we _enforce some behavioral invariants at compile time_, so compiler is able to _cut off incorrect behavior totally_?

Not always, but _yes_ in some cases. One possible way is to use [typestates][3] to represent (in types) a _sequence of states_ our type is able to be in, and to declare transitions (via functions) between these states. Doing so will allow compiler to __cut off incorrect state transitions at compile time__.

A real-world example of applying this idiom in [Rust] is the awesome [state_machine_future] crate.

For better understanding [typestates][3], read through the following articles:
- [David Teller: Typestates in Rust][3]
- [Cliff L. Biffle: The Typestate Pattern in Rust][5]
- [Ana Hobden: Pretty State Machine Patterns in Rust][4]
- [Will Crichton: Type-level Programming in Rust][6]




## Task

For the `Post` type described above, assume the following behavior in our application:
```
+-----+              +-------------+            +-----------+
| New |--publish()-->| Unmoderated |--allow()-->| Published |
+-----+              +-------------+            +-----------+
                           |                          |
                         deny()                    delete()
                           |       +---------+        |
                           +------>| Deleted |<-------+
                                   +---------+
```

Implement this behavior using [typestates idiom][3], so that calling `delete()` on `New` post (or calling `deny()` on `Deleted` post) will be a compile-time error.





[derive_more]: https://docs.rs/derive_more
[Rust]: https://www.rust-lang.org
[state_machine_future]: https://docs.rs/state_machine_future
[uom]: https://docs.rs/uom

[1]: https://github.com/rust-unofficial/patterns/blob/master/patterns/newtype.md
[2]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
[3]: https://yoric.github.io/post/rust-typestate
[4]: https://hoverbear.org/2016/10/12/rust-state-machine-pattern
[5]: http://cliffle.com/blog/rust-typestate
[6]: http://willcrichton.net/notes/type-level-programming
