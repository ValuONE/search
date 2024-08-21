pub struct SearchResult {
    pub filename: String,
    pub content: Vec<String>,
    pub line: Vec<i64>,
}

impl Default for SearchResult {
    fn default() -> Self {
        Self {
            filename: "".to_string(),
            content: vec![],
            line: vec![],
        }
    }
}