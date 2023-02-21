extern crate walkdir;

mod args;

use std::error::Error;
use std::path::Path;
use clap::Parser;

use jwalk::{Parallelism};
use crate::args::SearchArgs;
use crate::args::EntityType::{Locate, Find};
use std::time::{Duration, Instant};

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

pub fn run() -> Result<OperationResult, Box<dyn Error>> {
    let search_args = SearchArgs::parse();
    let config = convert(search_args);
    
    let result = search(
        config.is_locate,
        &*config.query_or_filename,
        config.case_insensitive,
        config.dir
    )?;

    Ok(result)
}

pub fn display_results(results: OperationResult) {
    if results.is_locate {
        println!("Found the specified filename {} time(s): \n", results.filename.len());
    } else {
        println!("Found the specified string {} time(s): \n", results.line.len());
    }

    for i in 0..results.filename.len() {
        if results.is_locate {
            println!("> {} \n", results.filename[i]);
        } else {
            println!("> {}", results.filename[i]);
            println!("--> in line {}: {} \n", results.line[i], results.content[i])
        }
    }
}

pub fn convert(args: SearchArgs) -> Config {
    let mut config: Config = Config {
        is_locate: false,
        query_or_filename: "".to_string(),
        case_insensitive: false,
        dir: vec![],
    };

    match args.entity_type {
        Locate(command) => {
            config.is_locate = true;
            config.query_or_filename = command.filename;

            match command.command {
                args::LocateSubcommand::All => {},
                args::LocateSubcommand::Dir(select) => {
                    config.dir.push(select.dir);
                }
            }
        },
        Find(command) => {
            config.query_or_filename = command.search_query;
            config.case_insensitive = command.search_case_insensitive;

            match command.command {
                args::FindSubcommand::All => {},
                args::FindSubcommand::Dir(select) => {
                    config.dir.push(select.dir);
                }
            }
        }
    }

    config
}

pub fn start_screen() {
    print!(r"

   _____ _________    ____  ________  __
  / ___// ____/   |  / __ \/ ____/ / / /
  \__ \/ __/ / /| | / /_/ / /   / /_/ /
 ___/ / /___/ ___ |/ _, _/ /___/ __  /
/____/_____/_/  |_/_/ |_|\____/_/ /_/
                                                
Thanks for using search! Mady by Valu

-------------------------------------------------------------------------

    ");
}

pub struct Config {
    pub is_locate: bool,
    pub query_or_filename: String,
    pub case_insensitive: bool,
    pub dir: Vec<String>,
}

pub struct SearchResult {
    line: Vec<i32>,
    content: Vec<String>
}

pub struct OperationResult {
    pub is_locate: bool,
    pub filename: Vec<String>,
    pub content: Vec<String>,
    pub line: Vec<i32>,
    pub files_count: u64,
}

pub fn search_case_sensitive(query: &str, contents: &str) -> SearchResult {
    let mut search_result = SearchResult {
        line: vec![],
        content: vec![],
    };

    let mut count: i32 = 0;
    
    for line in contents.lines() {
        count += 1;
        if line.contains(query) {
            search_result.line.push(count);
            search_result.content.push(line.to_string());
        }
    }

    search_result
}

pub fn search_case_insensitive(query: &str, contents: &str) -> SearchResult {
    let mut search_result = SearchResult {
        line: vec![],
        content: vec![],
    };

    let mut count: i32 = 0;
    let query = query.to_lowercase();
    
    for line in contents.lines() {
        count += 1;
        if line.to_lowercase().contains(&query) {
            search_result.line.push(count);
            search_result.content.push(line.to_string());
        }
    }

    search_result
}

pub fn search(is_locate: bool, query_or_filename: &str, case_insensitive: bool, mut search_dir: Vec<String>) -> Result<OperationResult, Box<dyn Error>> {
    let mut result = OperationResult {
        is_locate,
        filename: vec![],
        content: vec![],
        line: vec![],
        files_count: 0,
    };

    let mut not_locate = vec![];

    if search_dir.is_empty() {
        for name in get_drive_letters() {
            search_dir.push(name);
        }
    }

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );

    pb.set_message("Searching for files...");
    let started = Instant::now();

    for name in search_dir {
        jwalk::WalkDir::new(name)
            .parallelism(Parallelism::RayonNewPool(0))
            .into_iter()
            .filter_map(|e| {
                result.files_count += 1;
                let dir_entry = e.ok()?;
                if dir_entry.file_type().is_file() {
                    if is_locate && dir_entry.file_name() == query_or_filename {
                        result.filename.push(dir_entry.path().display().to_string());
                    }

                    if !is_locate {
                        not_locate.push(dir_entry.path().display().to_string())
                    }

                    return Some(true);
                }
                None
        }).count();
    }

    pb.finish_with_message(format!("Scanned through {} files in {} \n", result.files_count,HumanDuration(started.elapsed())));

    if !is_locate {
        println!("\n Searching in files now...\n");

        let pb = ProgressBar::new(result.files_count);

        for e in not_locate {
            pb.inc(1);
            if !is_locate {
                match fstream::contains(&e, query_or_filename) {
                    Some(b) => {
                        if b {
                            let search_result: SearchResult;
                            let contents = match fstream::read_text(&e) {
                                None => continue,
                                Some(value) => value
                            };

                            if case_insensitive {
                                search_result = search_case_insensitive(query_or_filename, &contents);
                            } else {
                                search_result = search_case_sensitive(query_or_filename, &contents)
                            }

                            if !search_result.line.is_empty() {
                                result.filename.push(e.to_string());

                                for i in 0..search_result.line.len() {
                                    let cur_content = search_result.content[i].clone();
                                    let cur_line = search_result.line[i];

                                    result.content.push(cur_content);
                                    result.line.push(cur_line);
                                }
                            }
                        }
                    }
                    None => continue,
                }
            }
        }

        pb.finish();

        println!("{} files searched in {:?}", result.files_count, pb.elapsed());
    }

    Ok(result)
}

fn get_drive_letters() -> Vec<String> {
    let mut exit_letter_vec = vec![];
    for x in HARD_DRIVE_NAMES {
        let path = Path::new(x);
        if path.exists() {
            exit_letter_vec.push(x.to_string());
        }
    }

    exit_letter_vec    
}

static HARD_DRIVE_NAMES: [&str; 26] = [
    "A:\\", "B:\\", "C:\\", "D:\\", "E:\\", 
    "F:\\", "G:\\", "H:\\", "I:\\", "J:\\", 
    "K:\\", "L:\\", "M:\\", "N:\\", "O:\\",
    "P:\\", "Q:\\", "R:\\", "S:\\", "Z:\\", 
    "U:\\", "V:\\", "W:\\", "X:\\", "Y:\\", 
    "Z:\\",
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![
                "safe, fast, productive."
            ],
            search_case_sensitive(query, contents).content
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents).content
        );
    }
}