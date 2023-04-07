mod model;
mod lib;

use crate::model::args::SearchArgs;
use clap::Parser;

fn main() {
    let test = SearchArgs::parse();

    println!("{:#?}", test);
}