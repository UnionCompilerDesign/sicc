use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

/// Locates the positions of top level expressions in a file
pub fn entry_points(path: &Path) -> Vec<usize> {
    let file: Result<File, std::io::Error> = File::open(path);

    match file {
        Ok(file) => {
            let reader: BufReader<File> = BufReader::new(file);
            let mut lines: Vec<usize> = Vec::new();

            for (index, line) in reader.lines().enumerate() {
                match line {
                    Ok(line) => {
                        let line: &str = line.trim_start();
                        if line.starts_with("import") || line.starts_with("pub") ||
                        line.starts_with("fn") || line.starts_with("struct") ||
                        line.starts_with("enum") {
                            lines.push(index)
                        }
                    }
                    Err(e) => {panic!("{:?}", e)}
                }
            }
            lines
        }
        _ => panic!("File not found")
    }
}