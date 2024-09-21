use clap::Parser;
use miyav::cli_args::MiyavArgs;

fn main() {
    let args = MiyavArgs::try_parse();
    if let Err(e) = args {
        println!("{}", e);
    } else {
        println!("{:#?}", args);
    }
}
