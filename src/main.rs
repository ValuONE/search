mod model;
mod lib;
mod test;

use crate::model::args::SearchArgs;
use clap::Parser;
use crate::lib::file_search;
use crate::lib::utility::get_drive_letter;

fn main() {
    let mut config = SearchArgs::parse().convert_to_config();

    let mut dirs: Vec<String> = vec![];
    match config.dir {
        None => {
            dirs = get_drive_letter();
        }
        Some(dir) => {
            dirs.push(dir);
        }
    }

    let final_result = file_search::search(dirs, &*config.query, config.search_type);

    println!("{:#?}", final_result);
}