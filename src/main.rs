use grep_rustico::file_handler::FileHandler;
use std::env;

const EXPECTED_ARG_COUNT: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != EXPECTED_ARG_COUNT {
        eprintln!("Invalid arguments, the format is: <expression> <file>");
        return;
    }

    let expression = &args[1];

    let file_name = &args[2];

    let file_handler = match FileHandler::new(file_name) {
        Ok(handler) => handler,
        Err(err) => {
            eprintln!("grep: {}: {}", file_name, err);
            return;
        }
    };

    if let Err(err) = file_handler.process_file(expression) {
        eprintln!("Error: {}", err);
    }
}
