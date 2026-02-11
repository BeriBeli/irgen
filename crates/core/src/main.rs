use std::path::PathBuf;

use irgen_core::processing::load_excel;

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("Usage: cargo run -p core -- <input.xlsx>");
        return;
    };

    let input = PathBuf::from(path);
    match load_excel(&input) {
        Ok(result) => {
            println!(
                "Loaded: {} (sheets: {})",
                result.file.display(),
                result.sheet_count.unwrap_or_default()
            );
        }
        Err(err) => {
            eprintln!("Failed to load excel: {err}");
            std::process::exit(1);
        }
    }
}
