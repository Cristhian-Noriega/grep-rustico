use crate::regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Module that handles the file reading and processing.
pub struct FileHandler {
    file: File,
}

impl FileHandler {
    /// Creates a new FileHandler with the file name.
    pub fn new(file_name: &str) -> io::Result<FileHandler> {
        let path = Path::new(file_name);
        let file = File::open(path)?;
        Ok(FileHandler { file })
    }

    /// Reads the file line by line and processes each line with the given expression.
    pub fn process_file(&self, expression: &str) {
        let reader = BufReader::new(&self.file);
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => match self.process_line(expression, &line) {
                    Ok(_) => (),
                    Err(err) => eprintln!("Error processing line: {}", err),
                },
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
    }

    /// Processes a single line with the given expression. Prints the matching lines.
    fn process_line(&self, expression: &str, line: &str) -> Result<(), String> {
        let regex = Regex::new(expression)?;
        match regex.match_expression(line) {
            Ok(result) => {
                if result {
                    println!("{}", line);
                }
                Ok(())
            }
            Err(err) => Err(format!("Error matching expression: {}", err)),
        }
    }
}
