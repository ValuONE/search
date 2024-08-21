use crate::model::search_result::SearchResult;

pub fn search(query: &str, content: &str, sensitive: bool) -> SearchResult {
    let mut search_result =  SearchResult::default();
    let mut line_count: i64 = 0;

    if !sensitive {
        let _ = query.to_lowercase();
        let _ = content.to_lowercase();
    }

    if content.contains(query) {
        for line in content.lines() {
            line_count += 1;

            if !line.contains(query) {
                continue;
            }
            
            search_result.content.push(line.to_string());
            search_result.line.push(line_count);
        }
    }

    search_result
}