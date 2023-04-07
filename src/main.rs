use std::process;

use colored::Colorize;
use j2e::{read_json, write_env, Args};

fn main() {
    let args = Args::parse();

    let variable_map = match read_json(&args.input) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}: {}", "Error".bold().red(), e);
            process::exit(1)
        }
    };
    let variable_count = variable_map.len();

    if let Err(e) = write_env(&args.output, variable_map) {
        eprintln!("{}: {}", "Error".bold().red(), e);
        process::exit(1)
    }

    println!(
        "✔ Wrote {} variables to {}",
        variable_count.to_string().bold().green(),
        args.output.to_str().unwrap().bold().green()
    );
}
