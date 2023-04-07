use std::fs::read_to_string;
use jwalk::{Parallelism};

use crate::lib::content_search;
use crate::model::config::SearchType;
use crate::model::config::SearchType::{FindString, LocateFile};
use crate::model::results::{FinalResult, SearchResult};

pub fn search(dirs: Vec<String>, query: &str, search_type: SearchType, sensitive: bool) -> FinalResult {
    let mut result = FinalResult::default();

    for dir in dirs {
        jwalk::WalkDir::new(dir)
            .parallelism(Parallelism::RayonNewPool(0))
            .into_iter()
            .filter_map(|e| {
                let dir_entry = e.ok()?;

                if dir_entry.file_type().is_file() {
                    result.file_count += 1;

                    if !sensitive && search_type == LocateFile && dir_entry.file_name() == query ||
                        sensitive && search_type == LocateFile && dir_entry.file_name().to_ascii_lowercase().as_os_str() == query.to_lowercase().as_str() {
                        result.results.push(SearchResult {
                            filename: vec![
                                dir_entry.path().display().to_string()
                            ],
                            ..SearchResult::default()
                        });
                    }

                    if search_type == FindString {
                        match read_to_string(dir_entry.path()) {
                            Ok(content) => {
                                let mut content_result = content_search::search(query, &*content, sensitive);

                                if !content_result.content.is_empty() {
                                    content_result.filename.push(
                                        dir_entry.path().display().to_string()
                                    );
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