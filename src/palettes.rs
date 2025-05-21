use anyhow::Result;
use serde_derive::Deserialize;

use std::fmt::Display;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct JsonPalette {
    pub name: String,
    pub author: String,
    pub colors: Vec<String>,
}

impl Display for JsonPalette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let author = if self.author == " " || self.author.is_empty() {
            String::new()
        } else {
            format!(" by {}", self.author)
        };
        f.write_fmt(format_args!(
            "{}{author} [{} Colors]",
            self.name,
            self.colors.len()
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct JsonPalettes {
    pub bit2: Vec<JsonPalette>,
    pub bit3: Vec<JsonPalette>,
    pub bit4: Vec<JsonPalette>,
    pub bit5: Vec<JsonPalette>,
    pub bit6: Vec<JsonPalette>,
    pub bit7: Vec<JsonPalette>,
    pub bit8: Vec<JsonPalette>,
    pub bit9: Vec<JsonPalette>,
    pub bit10: Vec<JsonPalette>,
    pub bit11: Vec<JsonPalette>,
    pub bit12: Vec<JsonPalette>,
    pub bit13: Vec<JsonPalette>,
    pub bit14: Vec<JsonPalette>,
    pub bit15: Vec<JsonPalette>,
    pub bit16: Vec<JsonPalette>,
    pub bit20: Vec<JsonPalette>,
    pub bit24: Vec<JsonPalette>,
    pub bit28: Vec<JsonPalette>,
    pub bit32: Vec<JsonPalette>,
    pub bit36: Vec<JsonPalette>,
    pub bit40: Vec<JsonPalette>,
    pub bit44: Vec<JsonPalette>,
    pub bit48: Vec<JsonPalette>,
    pub bit52: Vec<JsonPalette>,
    pub bit56: Vec<JsonPalette>,
    pub bit60: Vec<JsonPalette>,
    pub bit64: Vec<JsonPalette>,
    #[serde(rename = "moreThan64bit")]
    pub more: Vec<JsonPalette>,
}

impl Display for JsonPalettes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{} Palettes]", self.len()))
    }
}

impl JsonPalettes {
    pub fn len(&self) -> usize {
        let JsonPalettes {
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

pub fn parse_palettes() -> Result<JsonPalettes> {
    let contents = read_to_string("./palettes.json")?;
    let palettes: JsonPalettes = serde_json::from_str(&contents)?;
    println!("Finished parsing {palettes}");
    Ok(palettes)
}
