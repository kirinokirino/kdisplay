use std::{iter::Map, slice::Chunks};

use anyhow::Error;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::imageops::FilterType;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rgb::RGB8;
struct Screen {
    input_image: DynamicImage,
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub scale: usize,
}

impl Screen {
    pub fn new(input_image: DynamicImage, width: u32, height: u32) -> Self {
        let input_image = input_image.resize(width, height, FilterType::Nearest);
        let (width, height) = (
            input_image.width().try_into().unwrap(),
            input_image.height().try_into().unwrap(),
        );
        let scale = 3;
        let buffer = to_scaled_buffer(&input_image, scale);
        
        Self {
            input_image,
            width,
            height,
            buffer,
            scale,
        }
    }
}

fn to_scaled_buffer(image: &DynamicImage, scale: usize) -> Vec<u32> {
    let pixels: Vec<u32> = image.as_bytes().chunks(3).map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | (v[2] as u32)).collect();
    let mut scaled: Vec<u32> = Vec::with_capacity(pixels.len() * 4);
    let (width, height): (usize, usize) = (image.width().try_into().unwrap(), image.height().try_into().unwrap());
    for y in 0..height*scale {
        for x in 0..width*scale {
            scaled.push(pixels[(y/scale) * width + (x/scale)]);
        }
    }
    scaled
}

struct Palette {
    colors: Vec<RGB8>,
}

fn main() -> Result<(), Error> {
    let img = ImageReader::open("img.png")?.decode()?;

    let screen = Screen::new(img, 200, 200);
    let mut window = Window::new(
        "game",
        screen.width * screen.scale,
        screen.height * screen.scale,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(
                &screen.buffer,
                screen.width as usize * screen.scale,
                screen.height as usize * screen.scale,
            )
            .unwrap();
    }
    Ok(())
}
