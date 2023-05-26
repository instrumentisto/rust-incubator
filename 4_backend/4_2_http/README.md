Step 4.2: HTTP servers and clients
==================================

__Estimated time__: 1 day




## Web frameworks and libraries

[This Awesome Rust section][21] illustrates the current situation with [web frameworks][22] and libraries in [Rust] ecosystem quite well.


### Client

For performing HTTP requests there are two main crates: [reqwest] and [hyper].

[reqwest] provides a convenient, higher-level HTTP client in both flavours: sync and async.

[hyper] is a general-purpose async HTTP library, which provides client capabilities.

For better understanding and familiarity with [Rust] HTTP clients, read through the following articles:
- [Official `reqwest` crate docs][reqwest]
- [Official `hyper` crate docs][hyper]
- [Official `hyper` crate guides: Client][23]


### Server

Regarding _async_ HTTP server implementation, the most stable and production-ready are [actix-web] and [hyper] crates at the moment.

[actix-web] is a small, pragmatic, and extremely fast [Rust] web framework. It's build on top of [actix] actor abstractions (so provides good integration with sync world) and is [very careful about performance][24]. Its HTTP support is quite feature-rich and production-tested.

[hyper] is a general-purpose async HTTP library, which aims for _fast_ and _correct_ HTTP implementation for [Rust]. As this is a general-purpose library, using it directly for server implementation can feel quite low-level and unergonomic. However, there are numerous [web frameworks][22] built on top of [hyper], which provide friendly interface: [warp], [tower-web], [rocket].

For better understanding and familiarity with [Rust] HTTP servers, read through the following articles:
- [Official `actix-web` crate docs][actix-web]
- [Official `actix` crate guides: Server][26]
- [Official `hyper` crate guides: Server][25]





[actix]: https://docs.rs/actix
[actix-web]: https://docs.rs/actix-web
[hyper]: https://docs.rs/hyper
[reqwest]: https://docs.rs/reqwest
[rocket]: https://docs.rs/rocket
[Rust]: https://www.rust-lang.org
[tower-web]: https://docs.rs/tower-web
[warp]: https://docs.rs/warp

[21]: https://github.com/rust-unofficial/awesome-rust#web-programming
[22]: https://en.wikipedia.org/wiki/Web_framework
[23]: https://hyper.rs/guides/client/basic
[24]: https://www.techempower.com/benchmarks/#section=data-r16&hw=ph&test=plaintext
[25]: https://hyper.rs/guides/server/hello-world
[26]: https://actix.rs/docs/server
