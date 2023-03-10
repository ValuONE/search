use clap:: {
    Args,
    Parser,
    Subcommand,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SearchArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Locate a file on your computer
    Locate(LocateCommand),
    /// Find a string in a file
    Find(FindCommand),
}

#[derive(Debug, Args)]
pub struct LocateCommand {
    /// Enter the file name that should be located
    pub filename: String,
    #[clap(subcommand)]
    /// Specify the location further
    pub command: LocateSubcommand
}

#[derive(Debug, Subcommand)]
pub enum LocateSubcommand {
    /// Search in a specified directory
    Dir(SelectDirCommand),
    /// Search your whole computer (it takes long)
    All
}

#[derive(Debug, Args)]
pub struct SelectDirCommand {
    pub dir: String
}

#[derive(Debug, Args)]
pub struct FindCommand {
    /// Enter the string that should be searched
    pub search_query: String,
    /// Search case insensitive [default value = true]
    #[clap(short, long)]
    pub search_case_insensitive: bool,
    #[clap(subcommand)]
    /// Specify the search further
    pub command: FindSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum FindSubcommand {
    /// Search in a specified directory
    Dir(SelectDirCommand),
    /// Search your whole computer (it takes very (very) long)
    All
}