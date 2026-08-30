use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug, PartialEq)]
enum Level {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Parser)]
struct Cli {
    /// Log level to filter by
    level: String,
    /// Path to the log file
    path: String,
}

#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    level: Level,
    message: String,
}
fn main() {
    let cli = Cli::parse();
    let level = match parse_level(&cli.level) {
        Some(l) => l,
        None => {
            eprintln!("loglens: unknown level '{}'", &cli.level);
            std::process::exit(2);
        }
    };
    let path = &cli.path;
    match File::open(path) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(text) => {
                        if let Some(entry) = parse_line(&text) {
                            if entry.level == level {
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
