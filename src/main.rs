#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
use anyhow::Error;
use delta_e::DE2000;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use lab::Lab;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use nanorand::{Rng, WyRand};
use rgb::RGB8;
extern crate serde_derive;

mod palettes;
use palettes::parse_palettes;
struct Screen {
    input_image: DynamicImage,
    pub input_image_width: usize,
    pub input_image_height: usize,
    pub buffer: Vec<u32>,
    pub screen_scale: usize,
    pub palette: Palette,
}

impl Screen {
    pub fn new(
        input_image: &DynamicImage,
        input_resized_width: u32,
        input_resized_height: u32,
        screen_scale: usize,
        palette: Palette,
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
            palette,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (
            self.input_image_width * self.screen_scale,
            self.input_image_height * self.screen_scale,
        )
    }

    fn apply_palette_closest(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.palette.find_closest(*pixel).closest;
        }
    }
    fn apply_palette_proportional_mix(&mut self) {
        let mut rng = WyRand::new();
        for pixel in &mut self.buffer {
            let ColorMix {
                closest,
                alternative,
                mix,
            } = self.palette.find_closest(*pixel);
            if rng.generate::<f32>() > mix {
                *pixel = closest;
            } else {
                *pixel = alternative;
            }
        }
    }

    fn apply_palette_dithered(&mut self) {
        let (width, _) = self.size();
        for (i, pixel) in self.buffer.iter_mut().enumerate() {
            let (y, x) = (i / width, i % width);
            let ColorMix {
                closest,
                alternative,
                mix,
            } = self.palette.find_closest(*pixel);

            *pixel = dither(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                closest,
                alternative,
                mix,
            );
        }
    }
}

const DISPERSION_MATRIX_SIZE: u8 = 9;
const DISPERSED: [u8; DISPERSION_MATRIX_SIZE as usize] = [1, 7, 4, 5, 8, 3, 6, 2, 9];

pub fn dither(x: i32, y: i32, main_color: u32, alternative_color: u32, mix: f32) -> u32 {
    let idx_in_dispersion_matrix = ((x - y * 3).abs() % DISPERSION_MATRIX_SIZE as i32) as usize;
    let color_threshold =
        DISPERSED[idx_in_dispersion_matrix] as f32 / DISPERSION_MATRIX_SIZE as f32;

    if mix < color_threshold {
        main_color
    } else {
        alternative_color
    }
}
fn to_scaled_buffer(image: &DynamicImage, scale: usize) -> Vec<u32> {
    let pixels: Vec<u32> = image
        .as_bytes()
        .chunks(3)
        .map(|v| ((u32::from(v[0]) << 16) | ((u32::from(v[1])) << 8) | (u32::from(v[2]))))
        .collect();
    let mut scaled: Vec<u32> = Vec::with_capacity(pixels.len() * (scale * scale));
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
    colors: Vec<Lab>,
}

impl Palette {
    pub fn new(colors: &[[u8; 3]]) -> Self {
        Self {
            colors: colors.iter().map(Lab::from_rgb).collect(),
        }
    }
    pub fn from_rgb8(colors: &[RGB8]) -> Self {
        Self {
            colors: colors
                .iter()
                .map(|c| Lab::from_rgb(&[c.r, c.g, c.b]))
                .collect(),
        }
    }
    pub fn from_string(colors: &[String]) -> Self {
        let mut result = Vec::with_capacity(colors.len());
        for color in colors {
            let r = color.chars().take(2).collect::<String>();
            let g = color.chars().take(2).collect::<String>();
            let b = color.chars().take(2).collect::<String>();
            result.push(Lab::from_rgb(&[
                u8::from_str_radix(&r, 16).unwrap(),
                u8::from_str_radix(&g, 16).unwrap(),
                u8::from_str_radix(&b, 16).unwrap(),
            ]));
        }
        Self { colors: result }
    }
    pub fn find_closest(&self, color: u32) -> ColorMix {
        let target = Lab::from_rgba(&color.to_ne_bytes());
        let (mut closest_color, mut closest_delta) = (self.colors[0], 101.0);
        let (mut alternative_color, mut alternative_delta) = (self.colors[0], 101.0);
        for palette_color in &self.colors {
            let delta = DE2000::new(target, *palette_color);
            if delta < closest_delta {
                alternative_color = closest_color;
                alternative_delta = closest_delta;
                closest_color = *palette_color;
                closest_delta = delta;
            } else if delta < alternative_delta {
                alternative_color = *palette_color;
                alternative_delta = delta;
            }
        }
        ColorMix::new(
            closest_color,
            alternative_color,
            closest_delta / alternative_delta,
        )
    }
}

struct ColorMix {
    pub closest: u32,
    alternative: u32,
    mix: f32,
}

impl ColorMix {
    pub fn new(closest: Lab, alternative: Lab, mix: f32) -> Self {
        let closest = rgb_to_u32(&closest.to_rgb());
        let alternative = rgb_to_u32(&alternative.to_rgb());
        Self {
            closest,
            alternative,
            mix,
        }
    }
}

fn main() -> Result<(), Error> {
    let palettes = parse_palettes().unwrap();

    let mut current_palette = None;
    for i in 0..5 {
        let palette = &palettes.bit2[i];
        println!("{palette}");
        current_palette = Some(palette);
    }
    let palette = Palette::from_string(&current_palette.unwrap().colors);
    let img = ImageReader::open("img.png")?.decode()?;
    let (w, h, s) = (240, 120, 5); // (480, 360, 1);
    let mut screen = Screen::new(&img, w, h, s, palette);
    screen.apply_palette_dithered();
    let (screen_w, screen_h) = screen.size();
    let mut window = Window::new(
        "FLOATING",
        screen_w,
        screen_h,
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

#[must_use]
pub fn rgb_to_u32(rgb: &[u8; 3]) -> u32 {
    (u32::from(rgb[0]) << 16) | ((u32::from(rgb[1])) << 8) | (u32::from(rgb[2]))
}
