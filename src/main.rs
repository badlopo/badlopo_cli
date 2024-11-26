use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version)]
struct BadLopoCli {
    #[command(subcommand)]
    pub command: BadLopoCommands,
}

#[derive(Subcommand, Debug)]
pub enum BadLopoCommands {
    #[command(about = "Show detailed information about the project")]
    About,
}

// cli entry
fn main() {
    match BadLopoCli::try_parse() {
        Ok(BadLopoCli { command }) => {}
        Err(err) => {
            println!(
                "An error occurred while parsing the command line arguments!\n\n========== BADLOPO CLI ==========\n{err}"
            );
        }
    }
}
