//! Generates metadata fixture JSON files for the frontend.
//!
//! Run with: cargo run --bin generate-fixtures [-- --force]
//! Or via: make generate-fixtures [FORCE=1]
//!
//! `--force` re-downloads every logo instead of only missing ones and ones whose URL changed.

use scanopy::server::shared::fixtures::generate_ui_data_fixtures;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output_dir = parse_output_dir(&args);
    let force = args.iter().any(|a| a == "--force");
    generate_ui_data_fixtures(&output_dir, force);
}

fn parse_output_dir(args: &[String]) -> PathBuf {
    for i in 1..args.len() {
        if args[i] == "--output-dir" {
            if let Some(dir) = args.get(i + 1) {
                return PathBuf::from(dir);
            }
            eprintln!("Error: --output-dir requires a path argument");
            std::process::exit(1);
        }
    }
    PathBuf::from("../ui/src/lib/data")
}
