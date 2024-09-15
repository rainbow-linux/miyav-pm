use clap::{Subcommand, Parser};

/// Manage packages on a Rainbow Linux system
#[derive(Parser, Debug)]
#[command(name = "iris")]
#[command(about = "Manage packages on a Rainbow Linux system", long_about = None)]
pub struct IrisArgs {
    #[command(subcommand)]
    pub(crate) command: IrisSubcommand,

}

#[derive(Debug, Subcommand)]
pub enum IrisSubcommand {
    Pack,
    Extract,
    Publish,
    Install,
    Build,
    Clean,
    Purge,
    Uninstall,
    Search,
    Show,
    Update,
    Sync,
}