# loglens

A fast Rust command-line tool for filtering and summarizing multi-format application logs.

## Features

- Filter logs by level: `INFO`, `WARN`, `ERROR`, `DEBUG`, and `FATAL`
- Filter by message text with `--contains`
- Filter by relative age with `--since`, such as `10m` or `2h`
- Count log levels with `stats`
- Parse several common log formats:
  - bracketed logs used by this project
  - Hadoop/log4j-style logs
  - Spark logs
  - ZooKeeper logs
- Stream filter operations without loading the whole file
- Parallelize statistics parsing with Rayon

## Installation

From source:

```bash
cargo install --path .
```

Or run directly from a checkout:

```bash
cargo run -- filter error sample.log
```

## Usage

Filter by level:

```bash
loglens filter error sample.log
```

Filter by level and message text:

```bash
loglens filter error --contains payment sample.log
```

Only show entries from a recent time window:

```bash
loglens filter error --since 10m sample.log
```

Show counts for every supported level:

```bash
loglens stats sample.log
```

Display help:

```bash
loglens --help
```

## Supported formats

The parser currently supports the project's bracketed format and several real-world formats from Hadoop, Spark, and ZooKeeper logs. Unsupported or malformed lines are skipped rather than causing the entire operation to fail.

## Development

Run the test suite:

```bash
cargo test
```

Run formatting and lint checks:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Benchmark

The benchmark corpus contains 600,000 generated lines based on real LogHub samples. The current release-mode implementation parses approximately 1.3–1.4 million lines per second on the development machine used for this project. Results vary by hardware, operating system, filesystem cache, and workload.

## License

Licensed under the MIT License. See [LICENSE](LICENSE).
