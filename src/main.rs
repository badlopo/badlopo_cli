mod serve;

use crate::serve::{ServeImpl, ServeMode};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    #[command(about = "Show detailed information about the project.")]
    About,

    #[command(about = "Establish a local server to serve static resources.")]
    Serve {
        #[arg(
            short,
            long,
            help = "Specify the root directory of the server.",
            default_value = "."
        )]
        root: PathBuf,
        #[arg(
            short,
            long,
            help = "Specify the entry file of the server. You can use an absolute path or a relative path to the root directory.",
            default_value = "index.html"
        )]
        entry: PathBuf,
        #[arg(short, long, help = "Specify server port.", default_value = "80")]
        port: u16,
        #[arg(
            short,
            long,
            help = "Specify the server mode.",
            default_value = "mixed",
            ignore_case = true
        )]
        mode: ServeMode,
    },
}

// cli entry
fn main() {
    match BadLopoCli::try_parse() {
        Ok(BadLopoCli { command }) => match command {
            BadLopoCommands::About => println!("{}", ABOUT_CLI),
            BadLopoCommands::Serve {
                root,
                entry,
                port,
                mode,
            } => ServeImpl::handle(root, entry, port, mode),
        },
        Err(err) => println!("{err}"),
    }
}
