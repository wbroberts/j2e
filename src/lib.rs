use std::{
    collections::HashMap,
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use serde_json::Value;

pub mod command;
use command::Args;

type J2EResult<T> = Result<T, Box<dyn Error>>;

pub fn execute(args: &Args) -> J2EResult<usize> {
    let variable_map = read_json(&args.input)?;
    let variable_count = variable_map.len();

    write_env(&args.output, variable_map)?;

    Ok(variable_count)
}

fn read_json(path: &PathBuf) -> J2EResult<HashMap<String, Value>> {
    let mut content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut content)?;

    let map: HashMap<String, Value> = serde_json::from_str(&content)?;

    Ok(map)
}

fn write_env(path: &PathBuf, variables: HashMap<String, Value>) -> J2EResult<()> {
    let mut output_file = open_output(path)?;

    for (key, value) in variables {
        let parsed_value = parse_value(value)?;
        let env_value = format!("{}={}\n", key.to_uppercase(), parsed_value);
        output_file.write_all(env_value.as_bytes())?;
    }

    Ok(())
}

fn parse_value(value: Value) -> J2EResult<String> {
    let mut parsed_value = serde_json::to_string(&value)?;

    if value.is_string() {
        parsed_value = parsed_value.replace('"', "");
    }

    Ok(parsed_value)
}

fn open_output(path: &PathBuf) -> J2EResult<File> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    Ok(file)
}
