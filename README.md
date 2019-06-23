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

Execute `salmon init` command. Then, some example files are generated in the project directory.

```
$ salmon init usagi
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /articles/2019/06/23/example.md
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /codes/2019/06/23/example.rb
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /layouts/article.hbs
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /layouts/index.hbs
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /layouts/page.hbs
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /layouts/rss.hbs
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /layouts/tag.hbs
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /layouts/year.hbs
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /pages/example.md
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /partials/header.hbs
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /partials/menu.hbs
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /resources/images/sushi_salmon.png
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /resources/stylesheets/layout.sass
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /salmon.yaml
[2019-06-23T09:28:02Z INFO  salmon::initializer] Writing /docker-compose.yaml
[2019-06-23T09:28:02Z INFO  salmon::initializer] Your new Salmon project has been initialized!
[2019-06-23T09:28:02Z INFO  salmon::initializer] Now you can build with `salmon build` command after moved the pr
oject directory.
[2019-06-23T09:28:02Z INFO  salmon::initializer] Execute `docker-compose up nginx` if you want to open your site
in http://localhost:10080/.
```

You can build the new project with `salmon build` command.

```
$ cd usagi/
$ salmon build
[2019-06-23T09:28:14Z INFO  salmon::layout] Start to load layout files.
[2019-06-23T09:28:14Z INFO  salmon::layout] Loading a layout: "/home/mozamimy/tmp/usagi/layouts/index.hbs"
[2019-06-23T09:28:14Z INFO  salmon::layout] Loading a layout: "/home/mozamimy/tmp/usagi/layouts/article.hbs"
[2019-06-23T09:28:14Z INFO  salmon::layout] Loading a layout: "/home/mozamimy/tmp/usagi/layouts/tag.hbs"
[2019-06-23T09:28:14Z INFO  salmon::layout] Loading a layout: "/home/mozamimy/tmp/usagi/layouts/year.hbs"
[2019-06-23T09:28:14Z INFO  salmon::layout] Loading a layout: "/home/mozamimy/tmp/usagi/layouts/page.hbs"
[2019-06-23T09:28:14Z INFO  salmon::layout] Loading a layout: "/home/mozamimy/tmp/usagi/layouts/rss.hbs"
[2019-06-23T09:28:14Z INFO  salmon::partial] Loading a partial file: "/home/mozamimy/tmp/usagi/partials/header.h$
s"
[2019-06-23T09:28:14Z INFO  salmon::partial] Loading a partial file: "/home/mozamimy/tmp/usagi/partials/menu.hbs
```

Generated docker-compose.yaml helps you to open your first project with Web browser (http://localhost:10080/).

```
$ docker-compose up nginx
```

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
