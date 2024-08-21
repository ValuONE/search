use jwalk::Parallelism;
use std::fs::read_to_string;

use crate::model::config::SearchType;
use crate::model::config::SearchType::{FindString, LocateFile};
use crate::model::final_result::FinalResult;
use crate::model::search_result::SearchResult;
use crate::util::content_search;

pub fn search(dirs: Vec<String>, query: &str, search_type: SearchType, sensitive: bool) -> FinalResult {
    let mut result = FinalResult::default();
    result.search_type = search_type.clone();

    for dir in dirs {
        jwalk::WalkDir::new(dir)
            .parallelism(Parallelism::RayonNewPool(0))
            .into_iter()
            .filter_map(|e| {
                let dir_entry = e.ok()?;

                if !dir_entry.file_type().is_file() {
                    return None;
                }

                result.file_count += 1;

                let mut file_name = dir_entry.file_name().to_os_string();
                if !sensitive {
                    let _ = query.to_lowercase().as_str();
                    file_name = file_name.to_ascii_lowercase().to_os_string();
                }

                if search_type == LocateFile && file_name == query {
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

                return Some(true);
            }).count();
    }

    result
}