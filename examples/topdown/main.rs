#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565},
    primitives::Rectangle,
    prelude::*,
};
use tinybmp::Bmp;
use trowel::{App, AppResult, Buttons, Error};

const BMP_DATA: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/topdown/sprites/player.bmp"));

const SPRITE_COUNT: usize = 18;

#[derive(Clone, Copy)]
struct SpriteData {
    name: &'static str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

const SPRITE_DATA: [SpriteData; SPRITE_COUNT] = [
    // Idle
    SpriteData { name: "sprite1", x: 52, y: 21, width: 13, height: 21 },
    SpriteData { name: "sprite2", x: 112, y: 23, width: 13, height: 21 },
    SpriteData { name: "sprite3", x: 38, y: 0, width: 13, height: 21 },
    SpriteData { name: "sprite4", x: 30, y: 71, width: 13, height: 20 },
    SpriteData { name: "sprite5", x: 20, y: 92, width: 13, height: 20 },
    SpriteData { name: "sprite6", x: 74, y: 113, width: 13, height: 20 },

    // right idle
    SpriteData { name: "sprite7", x: 126, y: 44, width: 15, height: 21 },
    SpriteData { name: "sprite8", x: 126, y: 22, width: 15, height: 21 },
    SpriteData { name: "sprite9", x: 52, y: 43, width: 15, height: 21 },
    SpriteData { name: "sprite10", x: 0, y: 71, width: 15, height: 20 },
    SpriteData { name: "sprite11", x: 52, y: 0, width: 15, height: 20 },
    SpriteData { name: "sprite12", x: 96, y: 92, width: 15, height: 20 },

    // up idle
    SpriteData { name: "sprite13", x: 112, y: 68, width: 13, height: 21 },
    SpriteData { name: "sprite14", x: 112, y: 90, width: 13, height: 21 },
    SpriteData { name: "sprite15", x: 82, y: 44, width: 13, height: 21 },
    SpriteData { name: "sprite16", x: 21, y: 19, width: 13, height: 20 },
    SpriteData { name: "sprite17", x: 51, y: 92, width: 13, height: 20 },
    SpriteData { name: "sprite18", x: 82, y: 92, width: 13, height: 20 },
];

#[derive(Clone, Copy)]
struct Animation {
    name: &'static str,
    frame_count: usize,
    frame_indices: &'static [usize],
}

const IDLE: Animation = Animation {
    name: "idle",
    frame_count: 4,
    frame_indices: &[0, 1, 2, 3, 4, 5],
};

const RIGHT_IDLE: Animation = Animation {
    name: "IdleRight",
    frame_count: 4,
    frame_indices: &[6, 7, 8, 9, 10, 11],
};

const UP_IDLE: Animation = Animation {
    name: "IdleUp",
    frame_count: 2,
    frame_indices: &[12, 13, 14, 15, 16, 17],
};

fn sprite_data_new(i: usize) -> SpriteData {
    SPRITE_DATA[i % SPRITE_COUNT]
}

struct TopDown {
    frame: i32,
    bmp: Option<Bmp<'static, Rgb565>>,
    current_animation: Animation,
    current_frame_index: usize,
}

impl App for TopDown {
    fn init(&mut self) -> AppResult {
        self.bmp = Some(Bmp::from_slice(BMP_DATA).map_err(|e| Error::BmpErr(e))?);
        self.current_animation = IDLE;
        self.current_frame_index = 0;
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.frame += 1;

        // Update the animation state based on button input
        if buttons.contains(Buttons::W) {
            self.current_animation = UP_IDLE;
        } else if buttons.contains(Buttons::D) {
            self.current_animation = RIGHT_IDLE;
        } else {
            self.current_animation = IDLE;
        }

        // Update the current frame index
        self.current_frame_index = (self.current_frame_index + 1) % self.current_animation.frame_count;

        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        if self.frame == 0 {
            display.clear(Rgb565::WHITE)
                   .map_err(|_| Error::DisplayErr)?;

        }
        let nth_frame = 15;
        if self.frame % nth_frame != 0 {
            return Ok(());
        }
        let sprite_index = self.current_animation.frame_indices[self.current_frame_index];

        let sprite = sprite_data_new(sprite_index);
        let at = Point::new((160 - sprite.width as i32) / 2, (128 - sprite.height as i32) / 2);
        self.bmp
            .expect("no bmp set")
            .draw_sub_image(&mut display.translated(at),
            &Rectangle::new(Point::new(sprite.x, sprite.y), Size::new(sprite.width, sprite.height)))
            .map_err(|_| Error::DisplayErr)?;

        Ok(())
    }  
}

#[trowel::entry]
fn main() {
    trowel::run(TopDown { frame: -1, bmp: None, current_animation: IDLE, current_frame_index: 0 });
}