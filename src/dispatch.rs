use crate::cli_args;
use crate::cmds;

pub fn dispatch_command(args: cli_args::MiyavSubcommand) -> Result<(), String> {
    match args {
        cli_args::MiyavSubcommand::Clean => cmds::clean::clean(cli_args::MiyavSubcommand::Clean),
    }
}