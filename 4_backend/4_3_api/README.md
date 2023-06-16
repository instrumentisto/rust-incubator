Step 4.3: API servers, clients and tools
========================================

__Estimated time__: 1 day




## RESTful




## GraphQL

[GraphQL] is a [flexible][200] query language for [API]s, allowing to request data partially and compose multiple nested requests as a single one, seasoned with a schema having an [expressive][201] [type system][1] (comparing to other [API] schemas) and [very strong][202] [introspection][2] capabilities out-of-the-box.

One of the strongest parts of [GraphQL] is its [whole ecosystem][203] built around the language, allowing to auto-generate code from schema (or schema from code), have documentation directly from introspection, play interactively with any [API]s in playgrounds, easily mock them, and much, much more. __Once you've built your [GraphQL] schema, you have everything else ready-to-go.__

Another strong part of [GraphQL] is that its __protocol is [transport][204]-agnostic__, so the same schema and queries, used via [HTTP], are __easily reusable via [WebSocket]__, allowing to [stream data][205] with almost zero effort atop.

For more familiarity with [GraphQL], read through the following articles:
- [Introduction to GraphQL][206]
- [The Fullstack Tutorial for GraphQL][207]


### Server

For implementing a [GraphQL] server in [Rust], there are two major crates in its ecosystem: [`juniper`] (provides more static guarantees) and [`async-graphql`] (more feature-rich). Both __manifest code-to-schema approach__ (writing [Rust] code and later generating a [GraphQL] schema from it), because [Rust] type system is far more expressive than the [GraphQL] one.

[`juniper-from-schema`] crate, however, tries to take it in opposite direction, and to some degree successfully __provides schema-to-code approach__ (generating [Rust] code using [`juniper`] from a provided [GraphQL] schema).

For more familiarity with implementing [GraphQL] server in [Rust], read through the following articles:
- [Official `juniper` crate docs][`juniper`]
- [Juniper Book]
- [Official `juniper-from-schema` crate docs][`juniper-from-schema`]
- [Official `async-graphql` crate docs][`async-graphql`]
- [Async-graphql Book]


### Client

For making request to existing [GraphQL][GraphQL] [API]s, you don't necessarily need a special crate in [Rust] for trivial cases, just [any HTTP client][231] is capable to send a [simple query/mutation request][232].

However, if more static guarantees is needed, then the [`graphql-client`] crate may be used, providing the __query-to-code approach__ ([Rust] code is generated from [GraphQL] files defining queries).

[`cynic`] crate takes the __opposite code-to-query approach__ of generating a [GraphQL] query out of [Rust] code and validating it statically against a provided [GraphQL] schema.

For more familiarity with making [GraphQL] requests in [Rust], read through the following articles:
- [Official `graphql-client` crate description][`graphql-client`]
- [Official `cynic` crate docs][`cynic`]
- [Official `cynic` crate guide](https://cynic-rs.dev)




## gRPC




## Task

Rework [the task from the previous step](../4_2_http/README.md#task) in a ["thick client" paradigm][41]:
- Server represents a [REST]ful [API] with separate endpoints for each operation.
- [CLI] client parses commands by itself and makes accurate requests to the server [REST]ful [API].

It should be possible to perform all the operations via [cURL] (or any other [HTTP]/[API] client) directly on the [REST]ful [API] server, without using the [CLI] client.

Additionally, implement generation of [OpenAPI] specification out of you server [REST]ful [API] code, and generate [HTML] documentation from the generated [OpenAPI] spec.

Avoid architecture [over-engineering][42] for this task, just use simple, straightforward and obvious solutions.




[`async-graphql`]: https://docs.rs/async-graphql
[`cynic`]: https://docs.rs/cynic
[`graphql-client`]: https://github.com/graphql-rust/graphql-client
[`juniper`]: https://docs.rs/juniper
[`juniper-from-schema`]: https://docs.rs/juniper-from-schema
[API]: https://en.wikipedia.org/wiki/API
[Async-graphql Book]: https://async-graphql.github.io/async-graphql/en
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[cURL]: https://en.wikipedia.org/wiki/CURL
[GraphQL]: https://graphql.org
[HTML]: https://en.wikipedia.org/wiki/HTML
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[Juniper Book]: https://graphql-rust.github.io/juniper/master
[OpenAPI]: https://en.wikipedia.org/wiki/OpenAPI_Specification
[REST]: https://en.wikipedia.org/wiki/Representational_state_transfer
[Rust]: https://www.rust-lang.org
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

[1]: https://en.wikipedia.org/wiki/Type_system
[2]: https://en.wikipedia.org/wiki/Type_introspection
[200]: https://graphql.org/learn/queries
[201]: https://graphql.org/learn/schema
[202]: https://graphql.org/learn/introspection
[203]: https://github.com/chentsulin/awesome-graphql#tools
[204]: https://en.wikipedia.org/wiki/Transport_layer
[205]: https://www.apollographql.com/docs/react/data/subscriptions
[206]: https://graphql.org/learn
[207]: https://www.howtographql.com
[231]: ../4_2_http/README.md#client
[232]: https://graphql.org/learn/serving-over-http
[41]: https://en.wikipedia.org/wiki/Rich_client
[42]: https://en.wikipedia.org/wiki/Overengineering
