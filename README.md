# Salmon

[![CircleCI](https://circleci.com/gh/mozamimy/salmon.svg?style=svg)](https://circleci.com/gh/mozamimy/salmon)

A static site generator specialized for blogging and designed for rabbits. Not for human.

## Installation

### Docker (Recommended)

You can try quickly with Docker.

```sh
docker pull mozamimy/salmon:latest
docker run --rm salmon /usr/local/sbin/salmon --help
```

### Binary

Download a binary from [GitHub release page](https://github.com/mozamimy/salmon/releases). Linux and macOS are supported.

### Cargo

You can install with cargo command if you are Rust programmer.

```sh
cargo install salmon
```

## Usage

### Initialize Salmon project

(TBD)

### Create new article template and directory with `salmon new` command

You can start writing quickly with `salmon new` command. That create an article template and directories for codes and images.

```
$ salmon new awesome_article
[2019-06-02T08:10:16Z INFO  salmon::template_generator] Created a directory "./articles/2019/06/02"
[2019-06-02T08:10:16Z INFO  salmon::template_generator] Wrote an article template to "./articles/2019/06/02/awesome_article.md"
[2019-06-02T08:10:16Z INFO  salmon::template_generator] Created a directory "./codes/2019/06/02"
[2019-06-02T08:10:16Z INFO  salmon::template_generator] Created a directory "./resources/images/2019/06/02"
```

See `salmon new --help` to control the behavior. Salmon creates all templates and directories if there is no options.

### Log

You can control log level by `RUST_LOG` environment variable. The default value is `info`.

### Config

You should put YAML config file in project root directory.

```yaml
version: '1' # required
blog: # required
  site_root: 'https://example.com/', # required
  index_page: # optional
    entries_per_page: 10 # optional
  year_page: # optional
    entries_per_page: 15 # optional
  tag_page: # optional
    entries_per_page: 15 # optional
```

(TBD)

## License

MIT
