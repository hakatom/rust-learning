use std::env;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);  

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);}
    );

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}

