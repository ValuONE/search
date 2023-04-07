use crate::model::config::SearchType;
use crate::model::config::SearchType::{FindString, LocateFile};

pub struct FinalResult {
    pub file_count: i64,
    pub search_type: SearchType,
    pub results: Vec<SearchResult>
}

impl FinalResult {
    pub fn display(self) {
        if self.search_type == LocateFile {
            for (i, result) in self.results.into_iter().enumerate() {
                println!("      [{}] > {}\n\n", i + 1, result.filename);
            }

            return;
        }

        for (i, result) in self.results.into_iter().enumerate() {
            println!("      [{}] > {} in line(s): {:?}", i + 1, result.filename, result.line);
        }
    }
}

impl Default for FinalResult {
    fn default() -> Self {
        Self {
            search_type: FindString,
            file_count: 0,
            results: vec![]
        }
    }
}

pub struct SearchResult {
    pub filename: String,
    pub content: Vec<String>,
    pub line: Vec<i64>
}

impl Default for SearchResult {
    fn default() -> Self {
        Self {
            filename: "".to_string(),
            content: vec![],
            line: vec![]
        }
    }
}