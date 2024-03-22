use std::env;
use std::io;

mod file_handler;
fn main() -> io::Result<()>{
    let args: Vec<String>   = env::args().collect();

    if args.len() != 3 {
        eprint!("Error");
        return Ok(());
    }

    let expression_str = &args[1];

    let file_name = &args[2];

    // let file_handler = FileHandler::new(file_name)?;


    // file_handler.process_file(&expression_str)?;

    let file_handler = match file_handler::FileHandler::new(file_name) {
        Ok(handler) => handler,
        Err(_) => {
            eprintln!("grep: {}: No existe el archivo o el directorio", file_name);
            return Ok(());
        }
    };

    file_handler.process_file(&expression_str)?;

    Ok(())
    
}
