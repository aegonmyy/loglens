use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Level {
    Info,
    Warn,
    Error,
    Debug,
    Fatal,
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

        #[arg(long)]
        since: Option<String>,
    },
    /// Show counts per level
    Stats {
        /// Path to the log file
        path: String,
    },
}

#[derive(Debug)]
struct LogEntry {
    timestamp: NaiveDateTime,
    level: Level,
    message: String,
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Filter { level, path, since } => run_filter(level, path, since),
        Command::Stats { path } => run_stats(path),
    }
}
fn parse_line(line: &str) -> Option<LogEntry> {
    if let Some(entry) = parse_plain(line) {
        return Some(entry);
    }
    parse_log4j(line)
        .or_else(|| parse_spark(line))
        .or_else(|| parse_zookeeper(line))
}

fn parse_plain(line: &str) -> Option<LogEntry> {
    let res = line.strip_prefix("[");
    let res2 = res?.split_once("] ");
    let (timestamp, rest) = res2?;
    let (level, message) = rest.split_once(" ")?;
    Some(LogEntry {
        timestamp: NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S").ok()?,
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
        "FATAL" => Some(Level::Fatal),
        _ => None,
    }
}

fn run_filter(level: String, path: String, since: Option<String>) {
    let filter_level = match parse_level(&level) {
        Some(l) => l,
        None => {
            eprintln!("loglens: unknown level '{level}'");
            std::process::exit(2);
        }
    };
    let cutoff = compute_cutoff(since);
    match File::open(&path) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(text) => {
                        if let Some(entry) = parse_line(&text) {
                            if entry.level == filter_level {
                                let time_ok = match cutoff {
                                    Some(c) => entry.timestamp >= c, // cutoff exists: compare
                                    None => true,                    // no cutoff: keep it
                                };
                                if time_ok {
                                    println!("{text}");
                                }
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
    let start = Instant::now();
    let mut total: u64 = 0;
    let mut parsed: u64 = 0;
    let mut stats: HashMap<Level, u32> = HashMap::new();
    match File::open(&path) {
        Ok(file) => {
            for line in BufReader::new(file).lines() {
                match line {
                    Ok(text) => {
                        total += 1;
                        if let Some(ent) = parse_line(&text) {
                            parsed += 1;
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
    for level in [
        Level::Debug,
        Level::Info,
        Level::Error,
        Level::Warn,
        Level::Fatal,
    ] {
        println!("{:?} {}", level, stats.get(&level).unwrap_or(&0));
    }
    let elapsed = start.elapsed();
    eprintln!(
        "parsed {parsed}/{total} lines in {}ms ({:.0} lines/sec)",
        elapsed.as_millis(),
        parsed as f64 / elapsed.as_secs_f64()
    );
}

fn parse_since(s: &str) -> Option<Duration> {
    if let Some(minutes) = s.strip_suffix("m") {
        let mins: u64 = minutes.parse().ok()?;
        Some(Duration::from_secs(mins * 60))
    } else if let Some(hours) = s.strip_suffix("h") {
        let hrs: u64 = hours.parse().ok()?;
        Some(Duration::from_secs(hrs * 60 * 60))
    } else {
        None
    }
}

fn compute_cutoff(since: Option<String>) -> Option<NaiveDateTime> {
    let s = since?; // None? stop, return None
    let dur = parse_since(&s)?; // unparsable? stop, return None
    let now = Local::now().naive_local();
    Some(now - dur)
}

fn parse_log4j(line: &str) -> Option<LogEntry> {
    let (date, rest) = line.split_once(' ')?; // "2015-10-18" | "18:01:47,978 INFO ..."
    let (time, rest) = rest.split_once(' ')?; // "18:01:47,978" | "INFO [main] ..."
    let (secs, _millis) = time.split_once(',')?; // "18:01:47" | "978" (dropped)
    let (level, message) = rest.split_once(' ')?; // "INFO" | "[main] org.apache..."
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    let time = NaiveTime::parse_from_str(secs, "%H:%M:%S").ok()?;
    let timestamp = date.and_time(time);
    Some(LogEntry {
        timestamp,
        level: parse_level(level)?,
        message: message.to_string(),
    })
}

fn parse_spark(line: &str) -> Option<LogEntry> {
    let (date, rest) = line.split_once(' ')?;
    let (time, rest) = rest.split_once(' ')?;
    let date = NaiveDate::parse_from_str(date, "%Y/%m/%d").ok()?;
    let time = NaiveTime::parse_from_str(time, "%H:%M:%S").ok()?;
    let timestamp = date.and_time(time);
    let (level, message) = rest.split_once(" ")?;
    Some(LogEntry {
        timestamp,
        level: parse_level(level)?,
        message: message.to_string(),
    })
}

fn parse_zookeeper(line: &str) -> Option<LogEntry> {
    let (date, rest) = line.split_once(' ')?; // "2015-10-18" | "18:01:47,978 INFO ..."
    let (time, rest) = rest.split_once(" - ")?; // "18:01:47,978" | "INFO [main] ..."
    let (secs, _millis) = time.split_once(',')?; // "18:01:47" | "978" (dropped)
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    let time = NaiveTime::parse_from_str(secs, "%H:%M:%S").ok()?;
    let timestamp = date.and_time(time);
    let (level, message) = rest.split_once(' ')?; // "INFO" | "[main] org.apache..."
    Some(LogEntry {
        timestamp,
        level: parse_level(level)?,
        message: message.to_string(),
    })
}
