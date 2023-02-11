use ahash::AHashSet;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() {
    let filename = std::env::args().nth(1);
    let mut lines = AHashSet::default();
    let mut output = BufWriter::new(std::io::stdout());

    if let Some(filename) = filename {
        if Path::new(&filename).exists() {
            let file = OpenOptions::new()
                .read(true)
                .open(filename.clone())
                .unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(Result::ok) {
                lines.insert(line);
            }
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();
        let mut writer = BufWriter::new(file);
        for line in std::io::stdin().lock().lines().filter_map(Result::ok) {
            if lines.insert(line.clone()) {
                writeln!(writer, "{}", line).unwrap();
                writeln!(output, "{}", line).unwrap();
            }
        }
    } else {
        for line in std::io::stdin().lock().lines().filter_map(Result::ok) {
            if lines.insert(line.clone()) {
                writeln!(output, "{}", line).unwrap();
            }
        }
    }
}
