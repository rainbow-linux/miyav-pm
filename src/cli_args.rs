use std::path::PathBuf;

use clap::{Subcommand, Parser};

/// Manage packages on a Rainbow Linux system
#[derive(Parser, Debug)]
#[command(name = "miyav")]
#[command(about = "Manage packages on a Rainbow Linux system", long_about = None)]
pub struct MiyavArgs {
    #[command(subcommand)]
    pub(crate) command: MiyavSubcommand,

}

#[derive(Debug, Subcommand)]
pub enum MiyavSubcommand {
    /// Packages a new package for Miyav
    Pack {
        /// The source path to package.
        path: Option<PathBuf> 
    },
    /// Extracts a .nbar.zst Miyav package
    Extract {
        /// The path of the .nbar.zst to extract
        path: PathBuf,
        /// The target to extract the package into
        #[arg(short='C',long)]
        destination: Option<PathBuf>
    },
    /// Publishes a package to a Sunshine registry
    Publish {
        /// The package name to publish
        path: Option<PathBuf>,
        /// The registry to publish into
        #[arg(short='P',long)]
        registry: Option<String>
    },
    /// Installs given packages from the repository
    Install {
        /// The names of packages to install. 
        /// If none specified, equivalent to update.
        packages: Vec<String>,
        #[arg(short= 'S', long)]
        /// The rootfs to bootstrap a system into.
        bootstrap: Option<PathBuf>
    },
    /// Builds a package that was downloaded from source.
    Build {
        /// The path for packages to build. 
        /// If not specified, builds the current directory.
        path: Option<PathBuf>,
        #[arg(short='i', long)]
        /// Whether to install the package after build.
        install: bool
    },
    /// Cleans build artifacts of a from source package
    Clean,
    /// Purges the Miyav cache
    Purge,
    /// Removes the specified packages
    Remove {
        /// The list of packages to uninstall.
        /// If not specified, does nothing and exits unsuccessfully. 
        packages: Vec<String>
    },
    /// Searches for a set of keywords in the repositories.
    Search {
        /// The keywords to search for.
        keywords: Vec<String>
    },
    /// Shows information about a set of packages.
    Show { 
        /// The list of packages to show information about.
        packages: Vec<String> 
    },
    /// Updates the entire system, or a certain set of packages if specified.
    Update {
        /// The names of the packages to upgrade.
        /// If not specified, upgrades only a certain set of packages.
        packages: Vec<String>
    },
    /// Synchronises Sunshine registry indexes.
    Sync {
        /// List of registries to synchronise.
        /// If not specified, synchronises all registries.
        repos: Vec<String>
    },
}