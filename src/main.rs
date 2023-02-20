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

    println!("is locate: {} \n filenames: {:#?} \n content {:#?} \n lines {:#?} \n {}",
        result.is_locate,
        result.filename,
        result.content,
        result.line,
        result.files_count
    );
}
