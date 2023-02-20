mod args;

use std::process;
use minigrep::{OperationResult, run};

fn main() {
    minigrep::start_screen();

    let result: OperationResult = match run() {
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
        Ok(value) => {
            value
        }
    };

    minigrep::display_results(result);
}