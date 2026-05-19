use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "🦀 rust image toolkit gng 💔", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Opens a window to view the image
    View {
        /// path to the image file to display
        file: PathBuf,
    },
    /// Converts an image to a new format or compresses it
    Convert {
        /// path to the input image file
        input: PathBuf,
        /// path to save the converted image (e.g., output.jpg)
        output: PathBuf,
    },
}
