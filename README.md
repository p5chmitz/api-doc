![](assets/splash500.png)

What better way to illustrate my OpenAPI doc explorations than by building a fully functional REST service that auto-generates and serves integrated OAS documentation?

<hr>

# About
For a full suite of documentation refer to the [API Docs](https://www.headyimage.com/api-doc/intro/) module on my site.

## Command Line Parsing
This service uses [clap](https://github.com/clap-rs/clap?tab=readme-ov-file) for argument parsing. The clap crate works with cargo, but you'll need slightly modified syntax when running options with no commands. For example, `api-doc -h` or `cargo run -- -h` provides a list of top-level commands and options, and `api-doc <command> -h` or `cargo run <command> -h` returns information for a specific command.
