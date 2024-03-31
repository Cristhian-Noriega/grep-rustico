use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::regex::Regex;

pub struct FileHandler {
    file: File
}

impl FileHandler {
    pub fn new(file_name: &str) -> io::Result<FileHandler> {
        let path = Path::new(file_name);
        let file = File::open(&path)?;
        Ok(FileHandler { file })
    }

    pub fn process_file(&self, expression: &str) {
        let reader = BufReader::new(&self.file);
        for line in reader.lines() {
            let line = line;
            let line = line.unwrap_or_default();
            let words: Vec<&str> = line.split_whitespace().collect();
            for word in words {
                let regex = Regex::new(expression);
                match regex {
                    Ok(regex) => {
                        match regex.match_expression(word) {
                            Ok(result) => {
                                if result {
                                    println!("\x1b[31m{}\x1b[0m", word);
                                }
                            }
                            Err(err) => {
                                eprintln!("Error matching expression: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error creating regex: {}", err);
                    }
                }
        }
    }

    }
}



