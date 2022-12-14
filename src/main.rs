#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
use anyhow::Error;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rgb::RGB8;
struct Screen {
    input_image: DynamicImage,
    pub input_image_width: usize,
    pub input_image_height: usize,
    pub buffer: Vec<u32>,
    pub screen_scale: usize,
}

impl Screen {
    pub fn new(
        input_image: &DynamicImage,
        input_resized_width: u32,
        input_resized_height: u32,
        screen_scale: usize,
    ) -> Self {
        let input_image = input_image.resize(
            input_resized_width,
            input_resized_height,
            FilterType::Nearest,
        );
        let (input_image_width, input_image_height) = (
            input_image.width().try_into().unwrap(),
            input_image.height().try_into().unwrap(),
        );
        let buffer = to_scaled_buffer(&input_image, screen_scale);

        Self {
            input_image,
            input_image_width,
            input_image_height,
            buffer,
            screen_scale,
        }
    }
}

fn to_scaled_buffer(image: &DynamicImage, scale: usize) -> Vec<u32> {
    let pixels: Vec<u32> = image
        .as_bytes()
        .chunks(3)
        .map(|v| ((u32::from(v[0]) << 16) | ((u32::from(v[1])) << 8) | (u32::from(v[2]))))
        .collect();
    let mut scaled: Vec<u32> = Vec::with_capacity(pixels.len() * 4);
    let (width, height): (usize, usize) = (
        image.width().try_into().unwrap(),
        image.height().try_into().unwrap(),
    );
    for y in 0..height * scale {
        for x in 0..width * scale {
            scaled.push(pixels[(y / scale) * width + (x / scale)]);
        }
    }
    scaled
}

struct Palette {
    colors: Vec<RGB8>,
}

fn main() -> Result<(), Error> {
    let img = ImageReader::open("img.png")?.decode()?;

    let screen = Screen::new(&img, 200, 200, 3);
    let mut window = Window::new(
        "game",
        screen.input_image_width * screen.screen_scale,
        screen.input_image_height * screen.screen_scale,
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
                screen.input_image_width as usize * screen.screen_scale,
                screen.input_image_height as usize * screen.screen_scale,
            )
            .unwrap();
    }
    Ok(())
}
