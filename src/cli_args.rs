use std::path::PathBuf;

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
    Pack {
        path: PathBuf 
    },
    Extract {
        path: PathBuf,
    },
    Publish {
        path: Option<PathBuf>
    },
    Install {
        packages: Vec<String>
    },
    Build {
        path: Option<PathBuf>
    },
    Clean,
    Purge,
    Uninstall {
        packages: Vec<String>
    },
    Search {
        keywords: Vec<String>
    },
    Show{ 
        packages: Vec<String> 
    },
    Update {
        packages: Vec<String>
    },
    Sync,
}