use clap::{Parser, Subcommand};

const ABOUT_CLI: &str = r##"===== ===== ===== ===== ===== ===== ===== =====
 _                 _  _
| |__    __ _   __| || |  ___   _ __    ___
| '_ \  / _` | / _` || | / _ \ | '_ \  / _ \
| |_) || (_| || (_| || || (_) || |_) || (_) |
|_.__/  \__,_| \__,_||_| \___/ | .__/  \___/
                               |_|

Project: https://github.com/badlopo/badlopo_cli
===== ===== ===== ===== ===== ===== ===== ====="##;

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
        Ok(BadLopoCli { command }) => match command {
            BadLopoCommands::About => println!("{}", ABOUT_CLI),
        },
        Err(err) => println!("{err}"),
    }
}
