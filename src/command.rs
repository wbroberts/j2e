use std::{env, path::PathBuf, process};

use colored::Colorize;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub output: PathBuf,
}

impl Args {
    pub fn parse() -> Args {
        let mut args = env::args().skip(1);
        let count = args.len();

        if count == 0 {
            print_help();
            process::exit(0)
        }

        let input = args.next().unwrap();

        if count == 1 && is_option(&input) {
            parse_option(&input);
        }

        if count < 2 {
            eprintln!(
                "{}: invalid number of args. Expected {}. Received {}.\n",
                "Error".bold().red(),
                "2".yellow(),
                count.to_string().red()
            );
            print_help();
            process::exit(1)
        }

        let output = args.next().unwrap();

        let input = PathBuf::from(input);
        let output = PathBuf::from(output);

        Args { input, output }
    }
}

fn is_option(arg: &str) -> bool {
    arg.starts_with('-')
}

fn parse_option(arg: &str) {
    match arg {
        "--version" | "-v" => {
            eprintln!("{} {}", NAME, VERSION);
            process::exit(0)
        }
        "--help" | "-h" => {
            print_help();
            process::exit(0)
        }
        _ => {
            eprintln!(
                "{}: unexpected option.\n\nDid you mean {} or {}?",
                "Error".bold().red(),
                "--version".yellow(),
                "--help".yellow()
            );
            process::exit(1)
        }
    }
}

fn print_help() {
    eprintln!(
        "Creates env variables from a json object.\n\n{}: j2e <INPUT PATH> <OUTPUT PATH>",
        "Usage".bold().yellow(),
    );
    eprintln!(
        "\n{}:\n  {}, {}\n  {}, {}",
        "Options".bold().yellow(),
        "-v".green(),
        "--version".green(),
        "-h".green(),
        "--help".green()
    );
}
