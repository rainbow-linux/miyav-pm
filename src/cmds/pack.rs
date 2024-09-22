use std::{env, path::PathBuf};

use crate::cli_args;

pub fn pack(args: cli_args::MiyavSubcommand) -> Result<(), String> {
        if let cli_args::MiyavSubcommand::Pack { path } = args {
            let pack_source = 
            path
            .clone()
            .unwrap()
            .to_str()
            .to_owned()
            .unwrap_or(env::current_dir().unwrap().to_str().unwrap());
            println!("Packaging {pack_source}", pack_source=pack_source)
        } else {
            let current_dir = std::env::current_dir();
            let current_dir = current_dir.unwrap_or(PathBuf::from(env::current_dir().unwrap()));
        };
        Ok(())
}