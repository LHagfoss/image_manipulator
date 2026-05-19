use clap::Parser;

use crate::cli::Args;

mod cli;
mod decoder;
mod window;

fn main() {
    let args = Args::parse();

    let path_str = args.file.to_string_lossy();
    let img = decoder::load(&path_str);
    let (width, height, buffer) = decoder::convert(&img);

    window::run(&path_str, width, height, buffer);
}
