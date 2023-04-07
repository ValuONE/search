use std::fs::read_to_string;
use jwalk::{Parallelism};
use crate::lib::cli_feedback::setup_progress_indication;

use crate::lib::content_search;
use crate::model::config::SearchType;
use crate::model::config::SearchType::{FindString, LocateFile};
use crate::model::results::{FinalResult, SearchResult};

pub fn search(dirs: Vec<String>, query: &str, search_type: SearchType, sensitive: bool) -> FinalResult {
    let mut result = FinalResult::default();
    result.search_type = search_type.clone();

    for dir in dirs {
        jwalk::WalkDir::new(dir)
            .parallelism(Parallelism::RayonNewPool(0))
            .into_iter()
            .filter_map(|e| {
                let dir_entry = e.ok()?;

                if dir_entry.file_type().is_file() {
                    result.file_count += 1;

                    let dir_name = dir_entry.file_name();
                    if !sensitive {
                        query.to_lowercase().as_str();
                        let dir_name = dir_name.to_ascii_lowercase().as_os_str();
                    }

                    if search_type == LocateFile && dir_name == query {
                        result.results.push(SearchResult {
                            filename: dir_entry.path().display().to_string(),
                            ..SearchResult::default()
                        });
                    }

                    if search_type == FindString {
                        match read_to_string(dir_entry.path()) {
                            Ok(content) => {
                                let mut content_result = content_search::search(query, &*content, sensitive);

                                if !content_result.content.is_empty() {
                                    content_result.filename = dir_entry.path().display().to_string();
                                    result.results.push(content_result);
                                }
                            }
                            Err(_) => {}
                        };
                    }
                }

                return Some(true);
            }).count();
    }

    result
}