Step 2.9: Command-line arguments
================================

[Rust] provides a simple [`std::env::Arg`] iterator to access command-line arguments passed to a program. The basic idea of its usage is clearly described in [this chapter of Rust Book][1].

However, most of the time you require more advanced tool for that, which provides `--help` and `--version` flags out-of-the-box and a convenient way to set and describe you own options/flags. For that case there is the well-known [clap] crate in [Rust].

Read through [clap description][clap] and [official clap docs][clap docs] to understand its design, features and usage.




## Task

Write a simple program which acts as [handlebars] templates engine CLI (command-line interface) and allows to render templates by passing its values via command line.

Here is how CLI should look like:
```
$ cargo run -- --help
Handlebars CLI 0.1.0
Renders handlebars templates to STDOUT

USAGE:
    MyApp [FLAGS] [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -v, --val <name>=<value>    Set a value for template variable
    -o, --output <FILE>         Write rendering result into a file instead of STDOUT

ARGS:
    FILE    The template file to be rendered
```





[`std::env::Arg`]: https://doc.rust-lang.org/stable/std/env/struct.Args.html
[Rust]: https://www.rust-lang.org
[clap]: https://crates.io/crates/clap
[clap docs]: https://docs.rs/clap
[handlebars]: https://crates.io/crates/handlebars

[1]: https://doc.rust-lang.org/book/second-edition/ch12-01-accepting-command-line-arguments.html
