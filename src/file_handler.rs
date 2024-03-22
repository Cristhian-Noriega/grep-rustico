use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


pub struct FileHandler {
    file: File
}

impl FileHandler {
    pub fn new(file_name: &str) -> io::Result<FileHandler> {
        let path = Path::new(file_name);
        let file = File::open(&path)?;
        Ok(FileHandler { file })
    }

    pub fn process_file(&self, search_word: &str) -> io::Result<()> {
        let reader = BufReader::new(&self.file);
        for line in reader.lines() {
            let line = line?;
            let words: Vec<&str> = line.split_whitespace().collect();
            for word in words {
                if word.contains(search_word) {
                    println!("{}", word);
                }
            }
        }
        Ok(())
    }

}