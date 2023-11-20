use image::{imageops::FilterType, io::Reader};
use std::{error::Error, fmt::Display};

pub struct Config {
    pub file_name: String,
    pub size: u32,
    pub filter_type: FilterType,
    pub contrast_modifier: f32,
}

impl Config {
    pub fn new(
        file: &String,
        size: &String,
        filter_type: &String,
        contrast: &String,
    ) -> Result<Config, String> {
        let filter = match filter_type.to_lowercase().as_str() {
            "near" | "nearest" => FilterType::Nearest,
            "tri" | "triangle" => FilterType::Triangle,
            "cat" | "catmull" => FilterType::CatmullRom,
            "gaus" | "gaussian" => FilterType::Gaussian,
            "lan" | "lanczos3" => FilterType::Lanczos3,
            _ => return Err(String::from("Invalid filter type")),
        };

        Ok(Config {
            file_name: String::from(file.clone()),
            size: size
                .parse()
                .map_err(|_| String::from("Could not parse size!"))?,
            filter_type: filter,
            contrast_modifier: contrast
                .parse()
                .map_err(|_| String::from("Could not parse contrast"))?,
        })
    }
}

pub enum Shades {
    Black,
    Dark,
    Medium,
    Light,
    White,
}

impl Display for Shades {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shades::Black => f.write_str(" "),
            Shades::Dark => f.write_str("░"),
            Shades::Medium => f.write_str("▒"),
            Shades::Light => f.write_str("▓"),
            Shades::White => f.write_str("█"),
        }
    }
}
impl Shades {
    fn from_unit_value(value: f32) -> Shades {
        match value {
            x if x < 0.20 => Shades::Black,
            x if x < 0.40 => Shades::Dark,
            x if x < 0.60 => Shades::Medium,
            x if x < 0.80 => Shades::Light,
            _ => Shades::White,
        }
    }
}

pub fn raw_to_shades(width: u32, height: u32, raw: Vec<u8>) -> Vec<Shades> {
    let mut to_return: Vec<Shades> = Vec::new();

    for y in 0..height {
        if y == height - 1 {
            break;
        }

        if y % 2 == 1 {
            continue;
        }

        for x in 0..width {
            let value = (raw[(width * y + x) as usize] as f32
                + raw[(width * (y + 1) + x) as usize] as f32)
                / 512.0;
            to_return.push(Shades::from_unit_value(value));
        }
    }

    to_return
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let img = Reader::open(config.file_name)?
        .decode()?
        .resize(config.size, config.size, config.filter_type)
        .adjust_contrast(config.contrast_modifier)
        .unsharpen(0.0, 50)
        .into_luma8();

    let width = img.width();
    let height: u32 = img.height();
    let raw = img.into_vec();

    println!(
        "Width: {width} | Height: {height}, raw length:{}",
        raw.len()
    );

    let raw = raw_to_shades(width, height, raw);
    let height = raw.len() as u32 / width;

    for y in 0..height {
        for x in 0..width {
            print!("{}", raw[((y * width) + x) as usize]);
        }

        print!("\n");
    }

    Ok(())
}
