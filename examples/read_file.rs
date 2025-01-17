#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

extern crate alloc;

use alloc::boxed::Box;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use trowel::{App, AppResult, Buttons, Error, FS};

struct ReadFile {
    frame: i32, // Frame count

    file_contents: Box<str>,
}

impl App for ReadFile {
    fn read_write<F>(&mut self, fs: &mut F) -> AppResult
    where
        F: FS,
    {
        if self.frame != 1 {
            return Ok(());
        }

        let file = fs.read_file("hello.txt");
        if let Some((size, file)) = file {
            self.file_contents = Box::from(core::str::from_utf8(&file[..size]).unwrap());
        }

        Ok(())
    }

    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, _buttons: Buttons) -> AppResult {
        self.frame += 1;

        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.frame == 1 {
            // Create a new character style
            let style = MonoTextStyle::new(&ascii::FONT_7X13, Rgb565::WHITE);

            Text::new(&self.file_contents, Point::new(20, 30), style)
                .draw(display)
                .map_err(|_| Error::DisplayErr)?;
        }
        Ok(())
    }
}

fn main() {
    trowel::run(ReadFile {
        frame: 0,
        file_contents: Box::from("No filesystem"),
    });
}

#[cfg_attr(all(target_arch = "arm", target_os = "none"), cortex_m_rt::entry)]
fn entry() -> ! {
    main();
    loop {}
}
