# Learning notes — kept by your Rust companion

## Working agreements
- You write the code; I guide and review.
- When you say "wrote", I VERIFY: read the file, `cargo check`, and run it.
- Explain every new concept when it's introduced. No assumed knowledge.
- One tiny step + one concept at a time. If it's too dense, say "slower".

## Project
- loglens @ C:\Users\Alameen\projects\loglens
- experiments live in `examples/`, run with `cargo run --example <name>`

## Progress
- M0 (level filter, String-based) — COMPLETE, committed
- M1 (Level enum refactor) — COMPLETE, committed
- M2 (clap args) — COMPLETE, committed
- M3 (stats subcommand + HashMap) — COMPLETE, committed
- M4 (chrono timestamps + --since time filter) — COMPLETE, committed
- M5 (multi-format parsing on real LogHub data) — COMPLETE, committed: plain/log4j/spark/zookeeper, 2000/2000 on all three real files
- M6 (measure) — COMPLETE: Instant timing on stats, stderr report, gen_big.rs harness (100 cycles), verified 600,000/600,000 exact counts
- M7 (cut allocations) — IN PROGRESS
  - BASELINE (M6): 355,709 lines/sec (release, big.log 600k, ~1686ms)
  - CUT 1 DONE: removed format! from timestamp parsing (NaiveDate+NaiveTime+and_time) — now ~518-590k lines/sec (median), ~1.5-1.7x. Counts still exact.
  - CUT 2 NEXT: borrow the message (&str instead of String) — lifetimes for real
- M8 (parallelism: rayon, threads, Send/Sync) — PLANNED

## Real data
- testdata/ folder: LogHub 2k samples (hadoop, spark, zookeeper, hdfs, healthapp, openssh)
- none match our [YYYY-MM-DD HH:MM:SS] LEVEL msg format — that's the M5 feature

## Concepts covered so far
- env::args, collect, type annotation (Vec<String>)
- if, eprintln, exit codes (0/1/2), stderr vs stdout
- File::open, Result, match, Ok/Err
- BufReader, BufRead trait (why traits must be imported), .lines()
- struct, #[derive(Debug)], Option, Some/None
- &str vs String, .to_string()
- strip_prefix, split_once, tuple destructuring, ? on Option
- if let, struct field access, eq_ignore_ascii_case
- enum, #[derive(PartialEq)], match exhaustiveness, _ catch-all
- ? only in Option/Result-returning fns; match when handling in place
- expression-as-value (last expr = return, no ; no return)
- crates & crates.io, Cargo.toml deps, cargo add
- clap, #[derive(Parser)], /// doc comments, Cli::parse()
- clap subcommands, #[derive(Subcommand)], match dispatch
- HashMap, entry().or_insert(), get().unwrap_or()
- Copy vs Clone, ownership/moves (one owner, photocopy vs move)
- PartialEq vs Eq vs Hash (why HashMap needs Eq+Hash)
- derive: Debug, PartialEq, Eq, Hash, Copy, Clone
- chrono, NaiveDateTime, format strings (%Y-%m-%d), .ok() Result→Option
- Duration, strip_suffix, u64, Option<T> as optional clap flag
- E0382 moved value error — computing once outside loop vs clone-per-iteration
- closures |x| ... exist but beginner style = helper fn with ? + match
