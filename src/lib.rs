extern crate walkdir;

use std::fs;
use std::error::Error;
use std::path::Path;

use walkdir::WalkDir;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in 
        if config.case_insensitive {search(&config.query, &contents)} 
            else {search_case_insensitive(&config.query, &contents)} {
        println!("{}", line);
    }

    Ok(())
}

pub fn start_screen() {
    print!(r"
    
        __  ________   ________________  __________ 
       /  |/  /  _/ | / /  _/ ____/ __ \/ ____/ __ \
      / /|_/ // //  |/ // // / __/ /_/ / __/ / /_/ /
     / /  / // // /|  // // /_/ / _, _/ /___/ ____/ 
    /_/  /_/___/_/ |_/___/\____/_/ |_/_____/_/      
   
                                                
Thanks for using minigrep! Continue with your command or type help...

-------------------------------------------------------------------------

    ");
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
    pub search_all: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        fn has_single_component(path: &str) -> bool {
            !path.contains(std::path::is_separator)
        }

        let argument_case_insensitive = String::from("-C");
        let argument_search_all = String::from("-A");

        if args.len() < 2 {
            return Err("Not enough arguments provided");
        }

        let query = args[0].clone();
        let filename = args[1].clone();
        let case_insensitive = if args.contains(&argument_case_insensitive) {true} else {false};
        let search_all = if args.contains(&argument_search_all) {true} else {false};

        if search_all {
            match has_single_component(&filename) {
                false => {
                    return Err("When using '-A' only enter a single filename, not a path!");
                }
                true => ()
            };
        }
    
        Ok(Config {
            query,
            filename,
            case_insensitive,
            search_all
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn add_all_files_on_device() -> Vec<String> {

    let mut file_list = vec![];

    for name in get_drive_letters() {
        for e in WalkDir::new(name).into_iter().filter_map(|e| e.ok()) {
            if e.metadata().unwrap().is_file() {
                file_list.push(e.path().display().to_string());
            }
        }
    }

    let mut count: i32 = 0;
    for _item in &file_list {
        count += 1;
        //println!("{} named: {} ", count, item);
    }

    println!("{}", count);
    
    file_list
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}