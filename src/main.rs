mod hash;
mod image;
mod serve;

use crate::hash::{HashAlgorithm, HashImpl};
use crate::image::{ImageFormat, ImageImpl, ImageSize};
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

    #[command(about = "Calculate the hash value of the specified source.")]
    Hash {
        #[arg(help = "Source text or source file path to be evaluated.")]
        source: String,
        #[arg(short, long, help = "The hash algorithm to use.", ignore_case = true)]
        algorithm: HashAlgorithm,
        #[arg(
            short,
            long,
            help = "Whether to treat source as a raw string rather than a file path (default).",
            default_value = "false"
        )]
        raw: bool,
    },

    #[command(about = "Image-related processing. (metadata, resizing, format conversion, etc.)")]
    Image {
        #[arg(help = "Path to the source image.")]
        source: PathBuf,
        #[arg(
            short,
            long,
            help = "Target image format.\n- No format conversion will be performed if omitted.",
            ignore_case = true
        )]
        format: Option<ImageFormat>,
        #[arg(
            short,
            long,
            help = "Target image size.\n- This should be in the format of '<width>x<height>'.\n- If one of the width and height is omitted (\"<width>x\" or \"x<height>\"), the other will be scaled proportionally.\n- No resizing will be performed if omitted."
        )]
        size: Option<ImageSize>,
    },

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
            BadLopoCommands::Hash {
                source,
                raw,
                algorithm,
            } => HashImpl::handle(source, raw, algorithm),
            BadLopoCommands::Image {
                source,
                format,
                size,
            } => ImageImpl::handle(source, format, size),
            BadLopoCommands::Serve {
                root,
                entry,
                port,
                mode,
            } => ServeImpl::handle(root, entry, port, mode),
            // _ => {}
        },
        Err(err) => println!("{err}"),
    }
}

#[cfg(test)]
mod misc_test {
    #[test]
    fn t() {
        let s = "100x";

        if let [w, h] = s.split("x").collect::<Vec<&str>>()[..] {
            let w = w.parse::<u32>();
            let h = h.parse::<u32>();

            match (w, h) {
                (Ok(w), Ok(h)) => {
                    println!("Both {:?} {:?}", w, h);
                }
                (Ok(w), Err(_)) => {
                    println!("Width {:?}", w);
                }
                (Err(_), Ok(h)) => {
                    println!("Height {:?}", h);
                }
                (Err(_), Err(_)) => {
                    println!("None");
                }
            }
        } else {
            println!("None");
        }
    }
}
