#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
use delta_e::DE2000;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use lab::Lab;
use minifb::{Key, ScaleMode, Window, WindowOptions};

extern crate serde_derive;

mod palettes;
use palettes::parse_palettes;
struct Screen {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub palette: Palette,
}

impl Screen {
    pub fn new(input_image: &DynamicImage, w: u32, h: u32, palette: Palette) -> Self {
        let buffer = input_image
            .resize_exact(w, h, FilterType::Nearest)
            .to_rgb8()
            .pixels()
            .map(|pixel| rgb_to_u32(&[pixel[2], pixel[1], pixel[0]]))
            .collect();
        let (width, height) = (w as usize, h as usize);

        Self {
            width,
            height,
            buffer,
            palette,
        }
    }

    fn apply_palette_dithered(&mut self) {
        for (i, pixel) in self.buffer.iter_mut().enumerate() {
            let (y, x) = (i / self.width, i % self.width);
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
    let idx = ((x - y * 3).abs() as usize) % DISPERSION_MATRIX_SIZE as usize;
    let threshold = f32::from(DISPERSED[idx]) / f32::from(DISPERSION_MATRIX_SIZE);
    if mix < threshold { main_color } else { alternative_color }
}

struct Palette {
    colors: Vec<Lab>,
}

impl Palette {
    pub fn from_string(colors: &[String]) -> Self {
        Self {
            colors: colors
                .iter()
                .map(|c| {
                    let r = u8::from_str_radix(&c[0..2], 16).unwrap();
                    let g = u8::from_str_radix(&c[2..4], 16).unwrap();
                    let b = u8::from_str_radix(&c[4..6], 16).unwrap();
                    Lab::from_rgb(&[r, g, b])
                })
                .collect(),
        }
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
        ColorMix {
            closest: rgb_to_u32(&closest_color.to_rgb()),
            alternative: rgb_to_u32(&alternative_color.to_rgb()),
            mix: closest_delta / alternative_delta,
        }
    }
}

struct ColorMix {
    pub closest: u32,
    alternative: u32,
    mix: f32,
}

fn main() {
    let palettes = parse_palettes().unwrap();
    let palette = Palette::from_string(&palettes.bit8[9].colors);
    // let palette = Palette::from_string(&palettes.more[17].colors);
    println!("Palette: {}", palettes.more[17].name);

    let img = ImageReader::open("img.png").unwrap().decode().unwrap();
    let mut screen = Screen::new(&img, 640, 480, palette);
    screen.apply_palette_dithered();

    let mut window = Window::new(
        "FLOATING",
        screen.width,
        screen.height,
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
                screen.width as usize,
                screen.height as usize,
            )
            .unwrap();
    }
}

#[must_use]
pub fn rgb_to_u32(rgb: &[u8; 3]) -> u32 {
    u32::from(rgb[0]) | u32::from(rgb[1]) << 8 | u32::from(rgb[2]) << 16
}
