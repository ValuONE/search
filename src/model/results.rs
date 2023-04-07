#[derive(Debug)]
pub struct FinalResult {
    pub file_count: i64,
    pub results: Vec<SearchResult>
}

impl Default for FinalResult {
    fn default() -> Self {
        Self {
            file_count: 0,
            results: vec![]
        }
    }
}

#[derive(Debug)]
pub struct SearchResult {
    pub filename: Vec<String>,
    pub content: Vec<String>,
    pub line: Vec<i64>
}

impl Default for SearchResult {
    fn default() -> Self {
        Self {
            filename: vec![],
            content: vec![],
            line: vec![]
        }
    }
}