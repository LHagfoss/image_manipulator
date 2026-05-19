use clap::Parser;
use lagos_logger::{Colorize, info};

use crate::cli::{Args, Commands};
mod cli;
mod decoder;
mod window;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::View { file } => {
            let path_str = file.to_string_lossy();
            let img = decoder::load(&path_str);
            let (width, height, buffer) = decoder::convert(&img);

            window::run(&path_str, width, height, buffer);
        }
        Commands::Convert { input, output } => {
            let img = decoder::load(&input.to_string_lossy());

            decoder::save(&img, &output.to_string_lossy());
            info!("successfully converted and saved to {:?}", output);
        }
    }
}
