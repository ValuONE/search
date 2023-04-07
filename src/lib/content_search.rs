use crate::model::results::SearchResult;

pub fn search(query: &str, content: &str, sensitive: bool) -> SearchResult {
    let mut search_result =  SearchResult::default();
    let mut line_count: i64 = 0;

    if !sensitive { query.to_lowercase(); }

    for mut line in content.lines() {
        line_count += 1;
        if !sensitive { line.to_lowercase(); }

        if line.contains(query) {
            search_result.content.push(line.to_string());
            search_result.line.push(line_count);
        }
    }

    search_result
}