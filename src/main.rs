mod model;
mod lib;
mod test;

use std::time::Instant;
use crate::model::args::SearchArgs;
use clap::Parser;
use crate::lib::cli_feedback::setup_progress_indication;
use crate::lib::file_search;
use crate::lib::utility::get_drive_letter;

fn main() {
    let config = SearchArgs::parse().convert_to_config();

    let mut dirs: Vec<String> = vec![];
    match config.dir {
        None => {
            dirs = get_drive_letter();
        }
        Some(dir) => {
            dirs.push(dir);
        }
    }

    let pb = setup_progress_indication(format!("Searching for {}", config.query));

    let start_time = Instant::now();
    let final_result = file_search::search(dirs, &*config.query, config.search_type, config.case_sensitive);
    pb.finish_with_message(format!("Searched: {} files | Found {} match(es) | Duration: {:#?} \n", final_result.file_count, final_result.results.len(), start_time.elapsed()));

    final_result.display();
}