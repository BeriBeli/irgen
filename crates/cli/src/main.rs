use std::process::ExitCode;

use irgen_cli::{CliError, run};

fn main() -> ExitCode {
    match run(std::env::args_os().skip(1)) {
        Ok(Some(output)) => {
            println!("Generated {}", output.display());
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(CliError::Usage(message)) => {
            if message.starts_with("error:") {
                eprint!("{message}");
            } else {
                eprintln!("error: {message}");
            }
            ExitCode::FAILURE
        }
        Err(CliError::Runtime(message)) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
