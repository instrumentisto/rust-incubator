Step 3.9: Command-line arguments, environment variables and configs
===================================================================

__Estimated time__: 1 day




## CLI

[Rust] provides a _simple_ [`std::env::Arg`] iterator to access command-line arguments passed to a program.

However, most of the time you require more advanced tool for that, which provides `--help` and `--version` flags out-of-the-box and a _convenient way to setup and describe custom options/flags to build your own_ [CLI (command-line interface)][CLI]. For that case there is the well-known [clap] crate in [Rust] ecosystem.

The next level is [structopt] crate, which is built on top of [clap], and allows to define [CLI] in a _declarative and clean way_.

For better understanding and familiarity with [CLI] tools in [Rust] ecosystem, read through the following articles:
- [Rust Book: 12.1. Accepting Command Line Arguments][1]
- [Official `std::env::Arg` docs][`std::env::Arg`]
- [Official `clap` crate docs][clap]
- [Official `structopt` crate docs][structopt]




## Environment variables

[Rust] provides common primitives in [`std::env`] for working with [environment variables][2] as strings.

However, most of the time you want to operate with typed data, not with raw strings. Similarly to [structopt] for CLI, there is [envy] crate in [Rust] ecosystem, which uses [serde] as facade and allows to read data from environment variables in a _declarative and clean way_ (due to [serde attributes][4] support).

Finally, [dotenv] crate should be mentioned. It sets environment variables basing on `.env` file contents, which is widely used convention to simplify environment configuration and to not declare all required environment variables by hand each time when running a program. This one is especially _useful in development_.

For better understanding and familiarity with [environment variables][2] tools in [Rust] ecosystem, read through the following articles:
- [Rust Book: 12.5. Working with Environment Variables][3]
- [Official `std::env` docs][`std::env`]
- [Official `envy` crate docs][envy]
- [Official `dotenv` crate docs][dotenv]




## Configuration

For dealing with configurations there is a well-known [config] crate in [Rust] ecosystem, which simplifies creation and usage of hierarchical typed configuration structures in a [12-factor] way.

> Config lets you set a set of default parameters and then extend them via merging in configuration from a variety of sources:
> - Environment variables
> - Another Config instance
> - Remote configuration: etcd, Consul
> - Files: JSON, YAML, TOML, HJSON
> - Manual, programmatic override (via a `.set` method on the Config instance)
>
> Additionally, Config supports:
> - Live watching and re-reading of configuration files
> - Deep access into the merged configuration via a path syntax
> - Deserialization via `serde` of the configuration or any subset defined via a path

For better understanding and familiarity with [config] crate design, concepts, usage, and features, read through the following articles:
- [Official `config` crate docs][config]
- [`config` crate examples][5]




## Task

Write a simple program which prints out its actual configuration. Configuration should be implemented as a typed hierarchical structure, which is able to be parsed from a specified file and/or environment variables. 

The following priority should be applied (in ascending order) when merging:
1. Default values declared directly in [Rust] sources;
2. Values read from TOML file;
3. Values set by environment variables with `CONF_` prefix.

[CLI] of the program should look like:
```
$ cargo run -- --help
step_3_9 0.1.0
Prints its configuration to STDOUT.

USAGE:
    step_3_9 [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Enables debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --conf <conf>         Path to configuration file [env: CONF_FILE=]  [default: config.toml]
```





[12-factor]: https://12factor.net/config
[clap]: https://docs.rs/clap
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[config]: https://docs.rs/config
[dotenv]: https://docs.rs/dotenv
[envy]: https://docs.rs/envy
[Rust]: https://www.rust-lang.org
[serde]: https://docs.rs/serde
[`std::env`]: https://doc.rust-lang.org/std/env/index.html
[`std::env::Arg`]: https://doc.rust-lang.org/std/env/struct.Args.html
[structopt]: https://docs.rs/structopt

[1]: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
[2]: https://en.wikipedia.org/wiki/Environment_variable
[3]: https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html
[4]: https://serde.rs/attributes.html#field-attributes
[5]: https://github.com/mehcode/config-rs/tree/master/examples
