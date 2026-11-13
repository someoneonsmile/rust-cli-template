mod cli;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    if args.verbose {
        eprintln!("verbose mode on");
    }

    match args.file {
        Some(path) => println!("{{ project-name }}: {path}"),
        None => println!("{{ project-name }}: no input file"),
    }
}
