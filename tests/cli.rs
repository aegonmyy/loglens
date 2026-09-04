use std::process::Command;

#[test]
fn stats_reads_hadoop_file() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .args(["stats", "testdata/hadoop_2k.log"])
        .output()
        .expect("failed to run loglens");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");

    assert!(stdout.contains("Info 1040"));
    assert!(stdout.contains("Warn 808"));
    assert!(stdout.contains("Error 150"));
    assert!(stdout.contains("Fatal 2"));
}

#[test]
fn filter_rejects_unknown_level() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .args(["filter", "trace", "testdata/hadoop_2k.log"])
        .output()
        .expect("failed to run loglens");

    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8(output.stderr).expect("stderr should be valid UTF-8");

    assert!(stderr.contains("unknown level 'trace'"));
}

#[test]
fn filter_outputs_only_requested_level() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .args(["filter", "fatal", "testdata/hadoop_2k.log"])
        .output()
        .expect("failed to run loglens");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");

    let lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(lines.len(), 2);
    assert!(lines.iter().all(|line| line.contains("FATAL")));
}

#[test]
fn since_filters_old_lines() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .args(["filter", "error", "--since", "1m", "testdata/hadoop_2k.log"])
        .output()
        .expect("failed to run loglens");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");

    assert!(stdout.is_empty());
}
#[test]
fn contains_filters_by_message() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .args(["filter", "error", "--contains", "payment", "sample.log"])
        .output()
        .expect("failed to run loglens");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");

    let lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(lines.len(), 2);
    assert!(lines.iter().all(|line| line.contains("payment")));
}
