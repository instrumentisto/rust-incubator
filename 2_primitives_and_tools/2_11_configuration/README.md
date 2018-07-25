Step 2.11: Configuration
========================

There is a well-known [config] crate in [Rust] ecosystem which simplifies creation and usage of hierarchical typed configuration structures in a [12-factor] way.

Read through its [documentation][1] and see [usage examples][2]. 




## Task

Implement a hierarchical typed [config] structure for configuration described in the [`config.toml`](config.toml) file.

The following priority should be applied (in ascending order) for each configuration option:
1. Default value in [Rust] sources;
2. Value read from TOML file;
3. Value set by environment variable.





[Rust]: https://www.rust-lang.org
[config]: https://crates.io/crates/config
[12-factor]: https://12factor.net/config

[1]: https://docs.rs/config
[2]: https://github.com/mehcode/config-rs/tree/master/examples
