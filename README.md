# Wits

This tool is inspired by [Lucene's Luke](https://github.com/dmitrykey/luke).

It provides information about Tantivy's indices, such the list of fields, disk usage, ...etc.

## Installation

With cargo :

```shell
cargo install wits
```

## Subcommands

### `disk-usage`

Provides information about disk usage. Please note that [Tantivy client](https://github.com/quickwit-oss/tantivy-cli)
already offers this via `inspect` subcommand.

```shell
wits -t /tmp/tantivy-index/ disk-usage

wits -t /tmp/tantivy-index/ disk-usage title body
```

### `fields`

Provides information about fields.

* list : list all fields.

```shell
wits -t /tmp/tantivy-index/ fields list
```

* show : display information about a field.

```shell
wits -t /tmp/tantivy-index/ fields show title
```

### `interactive`

Allow to prompt for commands.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution