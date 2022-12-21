use serde_derive::Deserialize;
use serde_json;
#[macro_use]
use serde_derive;
use anyhow::{Error, Result};

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct Palette {
    name: String,
    author: String,
    colors: Vec<String>,
}

impl Display for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} by {} [{} Colors]",
            self.name,
            self.author,
            self.colors.len()
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct Palettes {
    bit2: Vec<Palette>,
    bit3: Vec<Palette>,
    bit4: Vec<Palette>,
    bit5: Vec<Palette>,
    bit6: Vec<Palette>,
    bit7: Vec<Palette>,
    bit8: Vec<Palette>,
    bit9: Vec<Palette>,
    bit10: Vec<Palette>,
    bit11: Vec<Palette>,
    bit12: Vec<Palette>,
    bit13: Vec<Palette>,
    bit14: Vec<Palette>,
    bit15: Vec<Palette>,
    bit16: Vec<Palette>,
    bit20: Vec<Palette>,
    bit24: Vec<Palette>,
    bit28: Vec<Palette>,
    bit32: Vec<Palette>,
    bit36: Vec<Palette>,
    bit40: Vec<Palette>,
    bit44: Vec<Palette>,
    bit48: Vec<Palette>,
    bit52: Vec<Palette>,
    bit56: Vec<Palette>,
    bit60: Vec<Palette>,
    bit64: Vec<Palette>,
    #[serde(rename = "moreThan64bit")]
    more: Vec<Palette>,
}

impl Display for Palettes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{} Palettes]", self.len()))
    }
}

impl Palettes {
    pub fn new() -> Self {
        Self {
            bit2: Vec::new(),
            bit3: Vec::new(),
            bit4: Vec::new(),
            bit5: Vec::new(),
            bit6: Vec::new(),
            bit7: Vec::new(),
            bit8: Vec::new(),
            bit9: Vec::new(),
            bit10: Vec::new(),
            bit11: Vec::new(),
            bit12: Vec::new(),
            bit13: Vec::new(),
            bit14: Vec::new(),
            bit15: Vec::new(),
            bit16: Vec::new(),
            bit20: Vec::new(),
            bit24: Vec::new(),
            bit28: Vec::new(),
            bit32: Vec::new(),
            bit36: Vec::new(),
            bit40: Vec::new(),
            bit44: Vec::new(),
            bit48: Vec::new(),
            bit52: Vec::new(),
            bit56: Vec::new(),
            bit60: Vec::new(),
            bit64: Vec::new(),
            more: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        let Palettes {
            bit2,
            bit3,
            bit4,
            bit5,
            bit6,
            bit7,
            bit8,
            bit9,
            bit10,
            bit11,
            bit12,
            bit13,
            bit14,
            bit15,
            bit16,
            bit20,
            bit24,
            bit28,
            bit32,
            bit36,
            bit40,
            bit44,
            bit48,
            bit52,
            bit56,
            bit60,
            bit64,
            more,
        } = self;

        bit2.len()
            + bit3.len()
            + bit4.len()
            + bit5.len()
            + bit6.len()
            + bit7.len()
            + bit8.len()
            + bit9.len()
            + bit10.len()
            + bit11.len()
            + bit12.len()
            + bit13.len()
            + bit14.len()
            + bit15.len()
            + bit16.len()
            + bit20.len()
            + bit24.len()
            + bit28.len()
            + bit32.len()
            + bit36.len()
            + bit40.len()
            + bit44.len()
            + bit48.len()
            + bit52.len()
            + bit56.len()
            + bit60.len()
            + bit64.len()
            + more.len()
    }
}

pub fn parse_palettes() -> Result<Palettes> {
    let contents = read_to_string("./palettes.json")?;
    let palettes: Palettes = serde_json::from_str(&contents)?;
    println!("Finished parsing {palettes}");
    Ok(palettes)
}
