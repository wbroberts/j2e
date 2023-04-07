use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    process,
};

use colored::Colorize;
use serde_json::Value;

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub output: PathBuf,
}

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

impl Args {
    pub fn parse() -> Args {
        let mut args = env::args().skip(1);
        let count = args.len();

        if count == 0 {
            write_help();
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
            write_help();
            process::exit(1)
        }

        let output = args.next().unwrap();

        let input = PathBuf::from(input);
        let output = PathBuf::from(output);

        Args { input, output }
    }
}

fn is_option(arg: &str) -> bool {
    arg.starts_with("-")
}

fn parse_option(arg: &str) {
    match arg.as_ref() {
        "--version" | "-v" => {
            eprintln!("{} {}", NAME, VERSION);
            process::exit(0)
        }
        "--help" | "-h" => {
            write_help();
            process::exit(0)
        }
        _ => {
            eprintln!(
                "{}: unexpected option.\n\nDid you mean --version or --help?",
                "Error".bold().red(),
            );
            process::exit(1)
        }
    }
}

pub fn write_help() {
    eprintln!(
        "Creates env variables from a json object.\n\n{}: {}",
        "Usage".bold(),
        "j2e <input file> <output file>"
    );
    eprintln!("\n{}:\n  -v --version\n  -h --help", "Options".bold());
}

pub fn read_json(path: &PathBuf) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut content)?;

    let map: HashMap<String, Value> = serde_json::from_str(&content)?;

    Ok(map)
}

pub fn write_env(path: &PathBuf, variables: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let mut output_file = open_output(&path)?;

    for (key, value) in variables {
        let parsed_value = parse_value(value)?;
        let env_value = format!("{}={}\n", key.to_uppercase(), parsed_value);
        output_file.write(env_value.as_bytes())?;
    }

    Ok(())
}

fn parse_value(value: Value) -> Result<String, Box<dyn Error>> {
    let mut parsed_value = serde_json::to_string(&value)?;

    if value.is_string() {
        parsed_value = parsed_value.replace("\"", "");
    }

    Ok(parsed_value)
}

pub fn open_output(path: &PathBuf) -> Result<File, Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    Ok(file)
}
