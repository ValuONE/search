mod args;
use args::MinigrepArgs;
use clap::Parser;

fn main() {
    let args = MinigrepArgs::parse();

    println!("{:?}", args);

    /*
    use std::process;
    use std::io;

    use minigrep::Config;
    use clap::Parser;

    minigrep::add_all_files_on_device();

    minigrep::start_screen();

    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();

    let split_input: Vec<&str> = user_input.split_whitespace().collect();
    let args: Vec<String> = split_input.iter().map(|&s|s.into()).collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing command: {}", err);
        process::exit(1);
    });
    
    println!("Searching for '{}' in '{}'", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
     */
}
