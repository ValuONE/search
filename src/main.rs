mod args;

use std::process;
use search::{OperationResult, run};

fn main() {
    search::start_screen();

    let result: OperationResult = match run() {
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
        Ok(value) => {
            value
        }
    };

    search::display_results(result);
}