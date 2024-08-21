mod model;
mod util;
mod test;

use crate::model::args::SearchArgs;
use clap::Parser;

fn main() {
    let config = SearchArgs::parse().convert_to_config();

    config.run()
}