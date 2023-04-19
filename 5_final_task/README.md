## To read

- [GraphQL](https://graphql.org/learn)
- [Juniper Book](https://graphql-rust.github.io/juniper/master/index.html)

## Task

Write simple [GraphQL] API server with the following data model:
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
- Add comprehensive documentation to you code;
- Cover your implementation with unit and E2E tests.
