use clap::ValueEnum;
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum ServeMode {
    Single,
    Mixed,
    Direct,
}
pub struct ServeImpl;

impl ServeImpl {
    pub fn handle(root: PathBuf, entry: PathBuf, port: u16, mode: ServeMode) {
        // TODO
        println!("{:?} | {:?} | {:?} | {:?}", root, entry, port, mode);
    }
}
