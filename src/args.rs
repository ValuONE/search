use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MinigrepArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Locate a file on your computer
    Locate(LocateCommand),
    /// Search for a string in a file
    Search(SearchCommand),
}

#[derive(Debug, Args)]
pub struct LocateCommand {
    pub filename: String,
    #[clap(subcommand)]
    pub command: LocateSubcommand
}

#[derive(Debug, Subcommand)]
pub enum LocateSubcommand {
    /// Search in a specified directory
    Dir(SelectDirCommand),
    /// Search your whole computer (might take a while)
    All
}

#[derive(Debug, Args)]
pub struct SelectDirCommand {
    pub dir: String
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    pub search_query: String,
    #[clap(subcommand)]
    pub command: SearchSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SearchSubcommand {
    /// Search in a specified directory
    Dir(SelectDirCommand),
    /// Search your whole computer (might take a while)
    All
}