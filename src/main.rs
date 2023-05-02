use std::process;

use colored::Colorize;
use j2e::{command::Args, execute};

fn main() {
    let args = Args::parse();

    match execute(&args) {
        Ok(variable_count) => {
            println!(
                "✔ Wrote {} variables to {}",
                variable_count.to_string().bold().green(),
                args.output.to_str().unwrap().bold().green()
            );
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".bold().red(), e);
            process::exit(1)
        }
    }
}
