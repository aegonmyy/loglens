# Learning notes — kept by your Rust companion

## Working agreements
- You write the code; I guide and review.
- When you say "wrote", I VERIFY: read the file, `cargo check`, and run it.
- Explain every new concept when it's introduced. No assumed knowledge.
- One tiny step + one concept at a time. If it's too dense, say "slower".

## Project
- loglens @ C:\Users\Alameen\projects\loglens
- experiments live in `examples/`, run with `cargo run --example <name>`

## Progress (M0) — COMPLETE
1. Read CLI args — DONE
2. Args guard + exit codes — DONE
3. File::open + Result/match — DONE
4. Read lines with BufReader — DONE
5. parse_line (struct + strip_prefix + split_once + ?) — DONE
6. Filter by level (if let + eq_ignore_ascii_case) — DONE

## Concepts covered so far
- env::args, collect, type annotation (Vec<String>)
- if, eprintln, exit codes (0/1/2), stderr vs stdout
- File::open, Result, match, Ok/Err
- BufReader, BufRead trait (why traits must be imported), .lines()
- struct, #[derive(Debug)], Option, Some/None
- &str vs String, .to_string()
- strip_prefix, split_once, tuple destructuring, ? on Option
- if let, struct field access, eq_ignore_ascii_case
