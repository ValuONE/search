use crate::util::file_search;
use indicatif::ProgressBar;
use std::time::Instant;

pub struct Config {
    pub search_type: SearchType,
    pub query: String,
    pub case_sensitive: bool,
    pub dirs: Vec<String>,
    pub progress_bar: Option<ProgressBar>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            search_type: SearchType::LocateFile,
            query: "".to_string(),
            case_sensitive: false,
            dirs: vec![],
            progress_bar: None,
        }
    }

    pub fn run(self) {
        let start_time = Instant::now();
        let final_result = file_search::search(self.dirs, &self.query, self.search_type, self.case_sensitive);
        self.progress_bar.unwrap().finish_with_message(
            format!("Searched: {} files | Found {} match{} | Duration: {:#?} \n",
                    final_result.file_count, final_result.results.len(),
                    if final_result.results.len() == 0 { "" } else { "es" },
                    start_time.elapsed())
        );

        final_result.display();
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SearchType {
    LocateFile,
    FindString,
}