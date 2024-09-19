use clap::{Subcommand, Parser};

/// Manage packages on a Rainbow Linux system
#[derive(Parser, Debug)]
#[command(name = "miyav")]
#[command(about = "Manage packages on a Rainbow Linux system", long_about = None)]
pub struct MiyavArgs {
    #[command(subcommand)]
    pub(crate) command: Option<MiyavSubcommand>,

}

#[derive(Debug, Subcommand)]
pub enum MiyavSubcommand {
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