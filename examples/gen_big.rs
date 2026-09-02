// gen_big.rs — build a large mixed-format log file for benchmarking.
// Usage: cargo run --release --example gen_big -- [cycles] [output]
//   cycles: how many times to repeat the 3 source files (default 100)
//   output: path to write (default testdata/big.log)
// 100 cycles x 3 files x 2000 lines = ~600k lines, ~80MB.

use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cycles: u32 = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);
    let out = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "testdata/big.log".to_string());

    let sources = [
        "testdata/hadoop_2k.log",
        "testdata/spark_2k.log",
        "testdata/zookeeper_2k.log",
    ];

    // Read each source once, then write it out `cycles` times.
    let contents: Vec<String> = sources
        .iter()
        .map(|path| fs::read_to_string(path).expect("cannot read source file"))
        .collect();

    let mut file = fs::File::create(&out).expect("cannot create output file");
    for i in 0..cycles {
        for content in &contents {
            file.write_all(content.as_bytes()).expect("write failed");
            if !content.ends_with('\n') {
                // source file had no trailing newline — add one so the next
                // file starts on a fresh line instead of merging into this one
                file.write_all(b"\n").expect("write failed");
            }
        }
        if i % 10 == 0 {
            eprintln!("cycle {i}/{cycles}");
        }
    }

    eprintln!("done: {out} ({cycles} cycles x {} files)", sources.len());
}
