//! This file provides functionality for processing a source file into a vector.

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

/// Locates the positions of top-level expressions in a file.
///
/// This function opens a file at the given path and reads through it line by line,
/// looking for lines that start with keywords indicative of top-level expressions such as
/// `import`, `pub`, `fn`, `struct`, and `enum`. It returns a vector containing the line numbers
/// where these expressions start.
///
/// # Parameters
/// * `path` - A reference to the path of the file to be analyzed.
///
/// # Returns
/// A vector of `usize` each representing the line number of a top-level expression.
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