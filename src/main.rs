use std::env;
use grep_rustico::regex::Regex;
use grep_rustico::file_handler::FileHandler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Invalid argurments");
        return;
    }

    let expression = &args[1];

    let file_name = &args[2];

    let file_handler = match FileHandler::new(file_name) {
        Ok(handler) => handler,
        Err(err) => {
            eprintln!("Error creating file handler: {}", err);
            return;
        }
    };

    file_handler.process_file(&expression);
}
