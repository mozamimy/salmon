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
