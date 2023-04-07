pub struct Config {
    pub search_type: SearchType,
    pub query: String,
    pub dir: Option<String>,
    pub case_sensitive: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search_type: SearchType::LocateFile,
            query: "".to_string(),
            dir: None,
            case_sensitive: false,
        }
    }
}

#[derive(PartialEq)]
pub enum SearchType {
    LocateFile,
    FindString
}