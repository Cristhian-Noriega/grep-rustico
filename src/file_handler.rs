use crate::error::RegexError;
use crate::match_result::MatchResult;
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
    pub fn process_file(&self, expression: &str) -> Result<(), RegexError> {
        let regex = match Regex::new(expression) {
            Ok(regex) => regex,
            Err(err) => return Err(err),
        };
        let reader = BufReader::new(&self.file);
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => match self.process_line(&regex, &line) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                },
                Err(_) => return Err(RegexError::InvalidFile),
            }
        }
        Ok(())
    }

    /// Processes a single line with the given expression. Prints the line if there's a match.
    fn process_line(&self, regex: &Regex, line: &str) -> Result<(), RegexError> {
        let result = regex.match_expression(line);
        match result {
            Ok(Some(match_result)) => {
                self.print_with_color(line, &match_result);
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(err) => Err(err),
        }
    }


    fn print_with_color(&self, line: &str, match_result: &MatchResult) {
        let (start, end) = match_result.range();
    println!(
        "{}\x1b[31m{}\x1b[0m{}",
        &line[..start],    
        &line[start..end],
        &line[end..]
    );
    }
}
