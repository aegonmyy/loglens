use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Level {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Filter log lines by level
    Filter {
        /// Log level to filter by
        level: String,
        /// Path to the log file
        path: String,
    },
    /// Show counts per level
    Stats {
        /// Path to the log file
        path: String,
    },
}

#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    level: Level,
    message: String,
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Filter { level, path } => run_filter(level, path),
        Command::Stats { path } => run_stats(path),
    }
    // let path = &cli.path;
}

fn parse_line(line: &str) -> Option<LogEntry> {
    let res = line.strip_prefix("[");
    let res2 = res?.split_once("] ");
    let (timestamp, rest) = res2?;
    let (level, message) = rest.split_once(" ")?;
    Some(LogEntry {
        timestamp: timestamp.to_string(),
        level: parse_level(level)?,
        message: message.to_string(),
    })
}

fn parse_level(s: &str) -> Option<Level> {
    match s.to_ascii_uppercase().as_str() {
        "INFO" => Some(Level::Info),
        "WARN" => Some(Level::Warn),
        "ERROR" => Some(Level::Error),
        "DEBUG" => Some(Level::Debug),
        _ => None,
    }
}

fn run_filter(level: String, path: String) {
    let filter_level = match parse_level(&level) {
        Some(l) => l,
        None => {
            eprintln!("loglens: unknown level '{level}'");
            std::process::exit(2);
        }
    };
    match File::open(&path) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(text) => {
                        if let Some(entry) = parse_line(&text) {
                            if entry.level == filter_level {
                                println!("{text}")
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("loglens: read error {err}");
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("loglens: cannot open '{path}': {err}");
            std::process::exit(1);
        }
    }
}

fn run_stats(path: String) {
    let mut stats: HashMap<Level, u32> = HashMap::new();
    match File::open(&path) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(text) => {
                        if let Some(ent) = parse_line(&text) {
                            *stats.entry(ent.level).or_insert(0) += 1;
                        }
                    }
                    Err(err) => {
                        eprintln!("loglens: read error {}", err);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("loglens: cannot open '{path}': {err}");
            std::process::exit(1);
        }
    };
    for level in [Level::Debug, Level::Info, Level::Error, Level::Warn] {
        println!("{:?} {}", level, stats.get(&level).unwrap_or(&0));
    }
    let _ = path;
}
