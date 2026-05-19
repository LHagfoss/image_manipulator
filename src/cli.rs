use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "🦀 rust image viewer gng 💔", long_about = None)]
pub struct Args {
    /// path to the image file to display 🦀
    pub file: PathBuf,
}
