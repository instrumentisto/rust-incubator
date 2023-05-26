Step 4: Backend ecosystem
=========================

__Estimated time__: 5 days

These steps describe common crates and tools in [Rust] ecosystem required for web backend development.

Before completing this step you should complete all its sub-steps. After doing them you should be able to answer the following questions:
- What should I use for HTTP server implementation in [Rust], when and why?
- How should I interact with databases in [Rust] application and why? How can I organize migrations for my project?
<!-- TODO -->




## Task

Write a simple [GraphQL] API server with the following data model:
- `User` has `id` (unique), `name` (unique) and `friends` (list of other `User`s) fields;
- `User` is able to authenticate with its `password`.

API requirements:
- Ability to register users;
- Ability to authenticate users;
- Ability to retrieve a single user and all its friends (without their friends) (should require authorization);
- Ability to add some user to friends list and remove from there (should require authorization).

Web frameworks, tools and database choices are up to you. Keep things simple to fit into dedicate time.

If you have enough time after implementing base requirements, consider to do the following for your solution:
- Provide migration for database schema (if possible);
- Add comprehensive documentation to your code;
- Cover your implementation with unit and E2E tests.




[GraphQL]: https://graphql.org/learn
[Rust]: https://www.rust-lang.org
