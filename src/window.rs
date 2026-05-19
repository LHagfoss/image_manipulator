use lagos_logger::{Colorize, info};
use minifb::{Key, Window, WindowOptions};

/// runs the window loop for the image viewer
pub fn run(title: &str, width: u32, height: u32, buffer: Vec<u32>) {
    let mut window = Window::new(
        title,
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            info!("esc hit, closing window...");

            break;
        }

        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap_or_else(|e| info!("failed to update window: {}", e));
    }
}
