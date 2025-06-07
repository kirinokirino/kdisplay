#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::use_self, clippy::module_name_repetitions)]

use delta_e::DE2000;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use lab::Lab;

use std::io::Write;

extern crate serde_derive;

mod palettes;
use palettes::parse_palettes;
struct Screen {
    pub width: usize,
    pub buffer: Vec<u8>,
    pub palette: Palette,
}

impl Screen {
    pub fn new(input_image: &DynamicImage, w: u32, h: u32, palette: Palette) -> Self {
        let buffer = input_image
            .resize_exact(w, h, FilterType::Nearest)
            .to_rgba8()
            .into_vec();

        Self {
            width: w as usize,
            buffer,
            palette,
        }
    }

    fn apply_palette_dithered(&mut self) {
        for (i, pixel) in self.buffer.chunks_exact_mut(4).enumerate() {
            let [r, g, b, a] = *pixel else {
                continue;
            }; // skip malformed pixels
            let (y, x) = (i / self.width, i % self.width);
            let ColorMix {
                closest,
                alternative,
                mix,
            } = self.palette.find_closest(r, g, b, a);

            let [r, g, b] = dither(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                closest,
                alternative,
                mix,
            );
            pixel.copy_from_slice(&[r, g, b, a]);
        }
    }

    pub fn display(&self) {
        let file = std::fs::File::options()
            .create(true)
            .read(true)
            .write(true)
            .open("/tmp/imagesink")
            .unwrap();
        let size = 640 * 480 * 4;
        file.set_len(size.try_into().unwrap()).unwrap();
        let mut mmap = unsafe { memmap2::MmapMut::map_mut(&file).unwrap() };
        if let Some(err) = mmap.lock().err() {
            panic!("{err}");
        }
        let _ = (&mut mmap[..]).write_all(&self.buffer.as_slice());
    }

    pub fn render(&self) -> image::RgbaImage {
        image::RgbaImage::from_raw(
            self.width as u32,
            (self.buffer.len() / 4 / self.width) as u32,
            self.buffer.clone(),
        )
        .expect("Failed to create image from buffer")
    }
}

const DISPERSION_MATRIX_SIZE: u8 = 9;
const DISPERSED: [u8; DISPERSION_MATRIX_SIZE as usize] = [1, 7, 4, 5, 8, 3, 6, 2, 9];
#[must_use]
pub fn dither(
    x: i32,
    y: i32,
    main_color: [u8; 3],
    alternative_color: [u8; 3],
    mix: f32,
) -> [u8; 3] {
    let idx = ((x - y * 3).unsigned_abs() as usize) % DISPERSION_MATRIX_SIZE as usize;
    let threshold = f32::from(DISPERSED[idx]) / f32::from(DISPERSION_MATRIX_SIZE);
    if mix < threshold {
        main_color
    } else {
        alternative_color
    }
}

struct Palette {
    colors: Vec<Lab>,
}

impl Palette {
    pub fn new(colors: &[String]) -> Self {
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
    pub fn find_closest(&self, r: u8, g: u8, b: u8, a: u8) -> ColorMix {
        let target = Lab::from_rgba(&[r, g, b, a]);
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
            closest: closest_color.to_rgb(),
            alternative: alternative_color.to_rgb(),
            mix: closest_delta / alternative_delta,
        }
    }
}

struct ColorMix {
    pub closest: [u8; 3],
    pub alternative: [u8; 3],
    pub mix: f32,
}

fn main() {
    let palettes = parse_palettes().unwrap();
    let img = ImageReader::open("img.png").unwrap().decode().unwrap();

    let mut palette_id = 0;
    loop {
        let palette = &palettes.more[palette_id];
        let mut screen = Screen::new(&img, 640, 480, Palette::new(&palette.colors));
        screen.apply_palette_dithered();
        println!("\nPalette: {}", palette);

        let mut counter = 0;
        while counter < 50 {
            screen.display();
            std::thread::sleep(std::time::Duration::from_millis(100));
            counter += 1;
            print!(".");
            std::io::stdout().flush().unwrap();
        }
        let img = screen.render();
        img.save("img.png").unwrap();
        palette_id = (palette_id + 1) % palettes.more.len();
    }
}
