pub mod cli;

use cli::{
    GenerateGridConfig, RemoveMatchingLinesConfig, ReplaceEntersConfig, StripWhitespacesConfig,
    TextLengthConfig, ToBlackAndWhiteConfig,
};
use image::{GenericImageView, ImageBuffer, ImageReader, Rgb, Rgba};
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub fn run_to_black_and_white(config: ToBlackAndWhiteConfig) -> Result<(), Box<dyn Error>> {
    let source_path = &config.source_path;
    let target_path = &config
        .target_path
        .unwrap_or(String::from("black_and_white_img.png"));

    let img = ImageReader::open(source_path)?.decode()?;

    log::debug!("Read image from:{source_path}");

    let black_and_white_img = img.grayscale();

    log::debug!("Turned image to grayscale");

    black_and_white_img.save(target_path)?;

    log::debug!("Saved image to:{target_path}");

    Ok(())
}

pub fn run_generate_grid(config: GenerateGridConfig) -> Result<(), Box<dyn Error>> {
    let source_path = &config.source_path;
    let target_path = &config.target_path.unwrap_or(String::from("pixel_grid.png"));

    let img = ImageReader::open(source_path)?.decode()?;

    let black_and_white_img = img.grayscale();

    let height = black_and_white_img.height();
    let width = black_and_white_img.width();

    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(width * 3, height * 3, Rgb([255, 255, 255]));

    let mut pixel_count = 0;

    black_and_white_img
        .pixels()
        .for_each(|pixel: (u32, u32, Rgba<u8>)| {
            let pixels_density = map_value_by_distribution(
                (255 - pixel.2[0]) as usize * pixel.2[3] as usize,
                |x| x,
                255 * 255,
                9,
            );

            pixel_count += pixels_density;

            let mut rng = thread_rng();

            let mut array: Vec<bool> = (0..9).map(|i| i < pixels_density).collect::<Vec<_>>();

            array.shuffle(&mut rng);

            array.iter().enumerate().for_each(|(index, &has_pixel)| {
                if !has_pixel {
                    return;
                }
                let p = Rgb([0u8, 0u8, 0u8]);
                imgbuf.put_pixel(
                    pixel.0 * 3 + index as u32 / 3,
                    pixel.1 * 3 + index as u32 % 3,
                    p,
                );
            });
        });

    imgbuf.save(target_path).unwrap();

    println!("The picture requires {} letters.", pixel_count);

    Ok(())
}

fn map_value_by_distribution<F>(
    value: usize,
    distribution: F,
    max_initial_value: usize,
    max_final_value: usize,
) -> usize
where
    F: Fn(f32) -> f32,
{
    let normalized_value = value as f32 / max_initial_value as f32;
    let distributed = distribution(normalized_value);
    let scaled_value = (distributed * max_final_value as f32).round() as usize;
    scaled_value.min(max_final_value)
}

pub fn run_text_length(config: TextLengthConfig) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.source_path)?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    let mut total_chars = 0;

    // Read chunks of 1024 bytes
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = buf_reader.read(&mut buffer) {
        if bytes_read == 0 {
            break; // End of file
        }
        contents.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
        total_chars += contents.chars().count();
        contents.clear(); // Clear contents to read next chunk
    }

    println!("The total number of characters: {}", total_chars);
    Ok(())
}

pub fn run_strip_whitespaces(config: StripWhitespacesConfig) -> Result<(), Box<dyn Error>> {
    let source_path = config.source_path;
    let target_path = config
        .target_path
        .unwrap_or(String::from("text_without_whitespaces.txt"));

    let source_file = File::open(source_path)?;
    let mut reader = BufReader::new(source_file);

    let target_file = File::create(target_path)?;
    let mut writer = BufWriter::new(target_file);

    let mut buffer = [0; 1024];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let result: String = buffer[..bytes_read]
            .iter()
            .filter_map(|&b| {
                let c = b as char;
                if !c.is_whitespace() {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
        writer.write_all(result.as_bytes())?;
    }

    writer.flush()?;
    Ok(())
}

pub fn run_replace_enters(config: ReplaceEntersConfig) -> Result<(), Box<dyn Error>> {
    let source_path = config.source_path;
    let target_path = config
        .target_path
        .unwrap_or(String::from("text_without_whitespaces.txt"));

    let source_file = File::open(source_path)?;
    let mut reader = BufReader::new(source_file);

    let target_file = File::create(target_path)?;
    let mut writer = BufWriter::new(target_file);

    let mut buffer = [0; 1024];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let result: String = buffer[..bytes_read]
            .iter()
            .map(|&b| {
                let c = b as char;
                if c != '\n' && c != '\r' {
                    c
                } else {
                    ' '
                }
            })
            .collect();
        writer.write_all(result.as_bytes())?;
    }

    writer.flush()?;
    Ok(())
}

pub fn run_remove_matching_lines(config: RemoveMatchingLinesConfig) -> Result<(), Box<dyn Error>> {
    let source_path = config.source_path;
    let target_path = config
        .target_path
        .unwrap_or(String::from("text_with_lines_removed.txt"));
    let pattern = Regex::new(&config.regex)?;

    let file = File::open(source_path)?;
    let reader = BufReader::new(file);

    let mut output = File::create(&target_path)?;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if !pattern.is_match(&line) {
            writeln!(output, "{}", line).unwrap();
        }
    });

    Ok(())
}
