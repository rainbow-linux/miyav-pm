use clap::Parser;
use miyav::cli_args::MiyavArgs;

fn main() {
    let args = MiyavArgs::try_parse().unwrap();
    println!("{:#?}", args);
}
