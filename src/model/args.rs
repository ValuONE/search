use clap::{
    Args,
    Parser,
    Subcommand,
};
use disk_name::get_letters;
use crate::model::config::{Config, SearchType};
use crate::util::cli_feedback::setup_progress_indication;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SearchArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

impl SearchArgs {
    pub fn convert_to_config(&self) -> Config {
        let mut config = Config::default();
        let d_letters = get_letters();

        match &self.entity_type {
            EntityType::Locate(locate_command) => {
                config.search_type = SearchType::LocateFile;
                config.query = locate_command.filename.clone();

                match &locate_command.command {
                    None => config.dirs = d_letters,
                    Some(val) => match val {
                        LocateSubcommand::Dir(dir_command) => config.dirs = vec![dir_command.dir.clone()],
                        LocateSubcommand::All => config.dirs = d_letters
                    }
                }
            }
            EntityType::Find(find_command) => {
                config.search_type = SearchType::FindString;
                config.query = find_command.search_query.clone();

                match &find_command.command {
                    None => config.dirs = d_letters,
                    Some(val) => match val {
                        FindSubcommand::Dir(dir_command) => config.dirs = vec![dir_command.dir.clone()],
                        FindSubcommand::All => config.dirs = d_letters
                    }
                }
            }
        }

        config.progress_bar = Some(setup_progress_indication(format!("Searching for {}", config.query)));

        config
    }
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
    /// The default search range is your whole computer. Use this arg to specify a directory if required
    pub command: Option<LocateSubcommand>
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
    pub dir: String,
}

#[derive(Debug, Args)]
pub struct FindCommand {
    /// Enter the string that should be searched
    pub search_query: String,
    #[clap(subcommand)]
    /// The default search range is your whole computer. Use this arg to specify a directory if required
    pub command: Option<FindSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum FindSubcommand {
    /// Search in a specified directory
    Dir(SelectDirCommand),
    /// Search your whole computer (it takes very (very) long)
    All,
}