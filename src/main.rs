use std::env;
use std::fs::write;
use std::process::exit;

fn main() {
    let github_output_path = match env::var("GITHUB_OUTPUT") {
        Ok(output) => {println!("File created successfully"); output},
        Err(e) => {eprintln!("Error: {e}"); return;},
    };

    let args: Vec<String> = env::args().collect();
    let error = &args[1];

    if !error.is_empty() {
        eprintln!("Error: {error}");
        match write(github_output_path, format!("error={error}\n")) {
            Ok(_) => {println!("Wrote to file")},
            Err(_) => {eprintln!("Could write to file")},
        };
        exit(1);
    }
}
