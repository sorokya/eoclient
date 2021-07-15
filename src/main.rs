extern crate sfml;

use sfml::{
    graphics::{Color, IntRect, RenderTarget, RenderWindow, Sprite, Texture, Transformable},
    system::Vector2f,
    window::ContextSettings,
    window::{Event, Style},
};

mod gfx_file;
use gfx_file::GfxFile;

pub fn main() {
    let gfx1 = GfxFile::new(1);
    const EO_WINDOW_SCALE: u32 = 3;
    const EO_WINDOW_SIZE: (u32, u32) = (640 * EO_WINDOW_SCALE, 480 * EO_WINDOW_SCALE);

    let mut window = RenderWindow::new(
        EO_WINDOW_SIZE,
        "Endless Online",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    window.set_vertical_sync_enabled(true);

    let texture =
        Texture::from_memory(gfx1.get_bitmap_at_id(132).unwrap(), &IntRect::default()).unwrap();

    let mut background = Sprite::new();
    background.set_texture(&texture, true);
    background.set_scale(Vector2f::new(3.0, 3.0));

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                _ => {}
            }
        }

        // Clear the window
        window.clear(Color::BLACK);

        window.draw(&background);

        // Display things on screen
        window.display()
    }
}
