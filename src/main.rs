use clap::Parser;
use lagos_logger::{Colorize, Level, logger};

use crate::cli::{Args, Commands};

mod cli;
mod display;
mod io;
mod transform;
mod window;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::View { file } => {
            let path_str = file.to_string_lossy();
            let img = io::load(&path_str);
            let (width, height, buffer) = display::convert(&img);

            window::run(&path_str, width, height, buffer);
        }
        Commands::Convert { input, output } => {
            let img = io::load(&input.to_string_lossy());

            io::save(&img, &output.to_string_lossy());
            logger!(
                Level::Success,
                "successfully converted and saved to {:?}",
                output
            );
        }
        Commands::Resize {
            input,
            width,
            height,
            output,
        } => {
            let img = io::load(&input.to_string_lossy());

            logger!(Level::Running, "resizing image to {}x{}...", width, height);
            let resized_img = transform::resize(&img, width, height);

            io::save(&resized_img, &output.to_string_lossy());
            logger!(
                Level::Success,
                "successfully resized and saved to {:?}",
                output
            );
        }
        Commands::Compress {
            input,
            quality,
            output,
        } => {
            let img = io::load(&input.to_string_lossy());

            logger!(
                Level::Running,
                "compressing image to {}% quality...",
                quality
            );
            let compressed_bytes = transform::compress(&img, quality);

            io::write_bytes(&output.to_string_lossy(), &compressed_bytes);
            logger!(
                Level::Success,
                "successfully compressed and saved to {:?}",
                output
            );
        }
    }
}
