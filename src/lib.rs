pub mod cli;

use cli::{
    CreateCustomImageConfig, FindDistributionConfig, GenerateGridConfig, RemoveMatchingLinesConfig,
    ReplaceEntersConfig, StripWhitespacesConfig, TextLengthConfig, ToBlackAndWhiteConfig,
};
use image::{GenericImageView, ImageBuffer, ImageReader, Rgb, Rgba};
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use svg::node::element::path::Data;
use svg::node::element::{Path, Style, Text};
use svg::Document;

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

    let grid_size = config.grid_size;
    let grid_area = grid_size * grid_size;

    let gamma = config.gamma;

    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(
        width * grid_size as u32,
        height * grid_size as u32,
        Rgb([255, 255, 255]),
    );

    let mut pixel_count = 0;

    black_and_white_img
        .pixels()
        .for_each(|pixel: (u32, u32, Rgba<u8>)| {
            let pixels_density = map_value_by_distribution(
                (255 - pixel.2[0]) as u32 * pixel.2[3] as u32,
                |x| x.powf(gamma),
                255 * 255,
                grid_area,
            );

            pixel_count += pixels_density;

            let mut rng = thread_rng();

            let mut array: Vec<bool> = (0..grid_area)
                .map(|i| i < pixels_density)
                .collect::<Vec<_>>();

            array.shuffle(&mut rng);

            array.iter().enumerate().for_each(|(index, &has_pixel)| {
                if !has_pixel {
                    return;
                }
                let p = Rgb([0u8, 0u8, 0u8]);
                imgbuf.put_pixel(
                    pixel.0 * grid_size + index as u32 / grid_size,
                    pixel.1 * grid_size + index as u32 % grid_size,
                    p,
                );
            });
        });

    imgbuf.save(target_path).unwrap();

    println!("The picture requires {} letters.", pixel_count);

    Ok(())
}

pub fn run_find_distribution(config: FindDistributionConfig) -> Result<(), Box<dyn Error>> {
    let source_path = &config.img_source_path;

    let img = ImageReader::open(source_path)?.decode()?;
    let black_and_white_img = img.grayscale();

    let total_chars = text_length(config.text_source_path)? as u32;

    let grid_area = config.grid_size * config.grid_size;

    log::debug!(
        "Starting calculation of the best gamma for text with {} total characters.",
        total_chars
    );

    let mut gamma_l = 0.0;
    let mut gamma_r = 100.0;
    let mut gamma = 1.0;

    let mut pixel_count = 0u32;

    // we want to always return the "over" value out of the two closest ones because
    // we want to fit the whole text in an image.
    let mut last_over: Option<(f32, u32)> = None;

    while gamma != gamma_l && gamma != gamma_r && pixel_count != total_chars {
        pixel_count = 0;

        black_and_white_img
            .pixels()
            .for_each(|pixel: (u32, u32, Rgba<u8>)| {
                let pixels_density = map_value_by_distribution(
                    (255 - pixel.2[0]) as u32 * pixel.2[3] as u32,
                    |x| x.powf(gamma),
                    255 * 255,
                    grid_area,
                );

                pixel_count += pixels_density;
            });

        log::debug!(
            "Gamma: {}, produced a pixel count of: {}",
            gamma,
            pixel_count
        );

        if pixel_count > total_chars {
            last_over = Some((gamma, pixel_count));
            gamma_l = gamma;
            gamma = (gamma_l + gamma_r) / 2.0;
        } else if pixel_count < total_chars {
            gamma_r = gamma;
            gamma = (gamma_l + gamma_r) / 2.0;
        } else {
            break;
        }
    }

    match last_over {
        Some(solution) => {
            println!(
                "The best gamma is: {} and it has an error of: {}",
                solution.0,
                solution.1 - total_chars
            );
        }
        None => {
            println!("No gamma can produce image dark enough for text of this size.")
        }
    }

    Ok(())
}

fn map_value_by_distribution<F>(
    value: u32,
    distribution: F,
    max_initial_value: u32,
    max_final_value: u32,
) -> u32
where
    F: Fn(f32) -> f32,
{
    let normalized_value = value as f32 / max_initial_value as f32;
    let distributed = distribution(normalized_value);
    let scaled_value = (distributed * max_final_value as f32).round() as u32;
    scaled_value.min(max_final_value)
}

fn text_length(source_path: String) -> Result<usize, Box<dyn Error>> {
    let file = File::open(source_path)?;
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

    Ok(total_chars)
}

pub fn run_text_length(config: TextLengthConfig) -> Result<(), Box<dyn Error>> {
    let total_chars = text_length(config.source_path)?;

    println!("The total number of characters: {}", total_chars);

    Ok(())
}

pub fn run_strip_whitespaces(config: StripWhitespacesConfig) -> Result<(), Box<dyn Error>> {
    let source_path = config.source_path;
    let target_path = config
        .target_path
        .unwrap_or_else(|| "text_without_whitespaces.txt".to_string());

    let source_file = File::open(source_path)?;
    let mut reader = BufReader::new(source_file);
    let target_file = File::create(target_path)?;
    let mut writer = BufWriter::new(target_file);

    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    let result: String = buffer.chars().filter(|c| !c.is_whitespace()).collect();

    writer.write_all(result.as_bytes())?;

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

pub fn run_create_custom_img(config: CreateCustomImageConfig) -> Result<(), Box<dyn Error>> {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let style = Style::new(
        "   .small {
        font: italic 13px sans-serif;
    }",
    );

    let text = Text::new("hello world")
        .set("x", 10)
        .set("y", 10)
        .set("class", "small");

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        // .add(path)
        .add(style)
        .add(text);

    svg::save("image.svg", &document).unwrap();
    Ok(())
}
