use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);  

    let (query, filepath) = parse_config(args);

    println!("Searching for {}", query);
    println!("In file {}", filepath);

    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    println!("with text:\n{contents}")
}

fn parse_config(args: Vec<String>) -> (&String, &String) {
    let query = &args[1];
    let filepath = &args[2];
    (query, filepath)
}
