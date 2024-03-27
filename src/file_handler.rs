use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::Chars;



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
                if matches_pattern(word, search_word) {
                    println!("{}", word);
                }
            }
        }
        Ok(())
    }

}



pub fn matches_pattern(word: &str, pattern: &str) -> bool {
    let mut word_chars = word.chars();
    let mut pattern_chars = pattern.chars();

    while let (Some(word_char), Some(pattern_char)) = (word_chars.next(), pattern_chars.next()) {
        match pattern_char {
            '.' => {
                // If the pattern character is '.', it matches any character, so continue to the next characters
            },
            _ => {
                // If the pattern character is not '.', it should match exactly with the corresponding character in the word
                if word_char != pattern_char {
                    return false;
                }
            }
        }
    }

    // Check if both iterators have reached the end
    word_chars.next().is_none() && pattern_chars.next().is_none()
}

