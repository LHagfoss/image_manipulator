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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("Failed to update window");
    }
}
