use clap::Parser;
use lagos_logger::{Colorize, Level, logger};

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
            logger!(Level::Success, "successfully converted and saved to {:?}", output);
        }
        Commands::Resize {
            input,
            width,
            height,
            output,
        } => {
            let img = decoder::load(&input.to_string_lossy());

            logger!(Level::Running, "resizing image to {}x{}...", width, height);
            let resized_img = decoder::resize(&img, width, height);

            decoder::save(&resized_img, &output.to_string_lossy());
            logger!(Level::Success, "successfully resized and saved to {:?}", output);
        }
    }
}
