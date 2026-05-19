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
    /// Converts an image to a new format
    Convert {
        /// path to the input image file
        input: PathBuf,
        /// path to save the converted image (e.g., output.jpg)
        output: PathBuf,
    },
    /// Resizes an image to a specific width and height
    Resize {
        /// path to the input image file
        input: PathBuf,
        /// target width (x) in pixels
        width: u32,
        /// target height (y) in pixels
        height: u32,
        /// path to save the resized image
        output: PathBuf,
    },
    /// Compresses an image to a smaller file size (JPEG usually)
    Compress {
        /// path to the input image file
        input: PathBuf,
        /// quality percentage (1-100, where 100 is best quality / largest file)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=100))]
        quality: u8,
        /// path to save the compressed image
        output: PathBuf,
    },
}
