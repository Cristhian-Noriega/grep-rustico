use std::env;
use grep_rustico::regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Invalid argurments");
        return;
    }

    let pattern = &args[1];

    let file_name = &args[2];
    
    let regex = Regex::new(pattern);

    


    
}
