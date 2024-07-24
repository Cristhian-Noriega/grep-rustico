# grep-rustico

Grep Rustico is a command-line tool inspired by the classic Unix/Linux `grep` utility. It's designed to search for lines in a file that match a specified regular expression pattern. This project aims to implement a simplified version of `egrep` (extended grep) functionality.

## Features

- Search for patterns in text files using regular expressions
- Support for various metacharacters and character classes
- Ability to handle complex regex patterns including concatenation, alternation, and precedence

## Usage

```
cargo run -- <regex_pattern> <file_path>
```

## Documentation

To view the documentation for this project, run the following command in the project directory:

```
cargo doc --open
```
