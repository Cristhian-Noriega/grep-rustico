use grep_rustico::file_handler::FileHandler;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
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
