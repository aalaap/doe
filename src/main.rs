use clap::App;
use glob::glob;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about("Manage .env files in the current directory")
        .subcommand(App::new("list").about("List available .envs"))
        .subcommand(App::new("current").about("Show the current configured .env"))
        .get_matches();

    match matches.subcommand_name() {
        Some("list") => list_files()?,
        Some("current") => current_env()?,
        _ => println!("Invalid command"),
    }

    Ok(())
}

fn list_files() -> Result<(), Box<dyn std::error::Error>> {
    let current_env = get_current_env()?; // Get the current environment

    let entries_iter = glob("*.env*").map_err(|err| Box::new(err) as Box<dyn std::error::Error>);

    for entry in entries_iter? {
        let entry = entry?;
        let file_name = entry.file_name().ok_or("Invalid file name")?;
        let file_name_str = file_name.to_str().ok_or("Invalid file name")?;

        let filename_without_env = file_name_str.trim_start_matches(".env");

        if filename_without_env == format!(".{}", current_env) {
            print!("* ");
        }

        println!("{}", filename_without_env);
    }

    Ok(())
}

fn get_current_env() -> Result<String, io::Error> {
    let doe_file_path = Path::new(".doe");

    if let Ok(file) = File::open(&doe_file_path) {
        let mut lines = io::BufReader::new(file).lines();
        if let Some(Ok(line)) = lines.next() {
            return Ok(line);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Unable to read .doe file",
    ))
}

fn current_env() -> Result<(), Box<dyn std::error::Error>> {
    println!("* {:?}", get_current_env()?);
    Ok(())
}
