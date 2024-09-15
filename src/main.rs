use clap::Parser;
use iris_pm::cli_args::IrisArgs;

fn main() {
    let args = IrisArgs::try_parse().unwrap();
    println!("{:#?}", args);
}
