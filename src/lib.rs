pub mod cli;

use cli::{GenerateGridConfig, TextLengthConfig, ToBlackAndWhiteConfig, TutorialConfig};
use image::{GenericImageView, ImageBuffer, ImageReader, Rgb, Rgba};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, Read};

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

    let mut grid_layout = Vec::<(bool, u32, u32)>::new();

    black_and_white_img
        .pixels()
        .for_each(|pixel: (u32, u32, Rgba<u8>)| {
            let pixels_density = map_value_by_distribution(
                (255 - pixel.2[0]) as usize * pixel.2[3] as usize,
                |x| x.sqrt(),
                255 * 255,
                9,
            );

            let mut rng = thread_rng();

            let mut array: Vec<bool> = (0..9).map(|i| i < pixels_density).collect::<Vec<_>>();

            array.shuffle(&mut rng);

            let f_array = array
                .iter()
                .enumerate()
                .map(|(index, &has_pixel)| {
                    (
                        has_pixel,
                        pixel.0 * 3 + index as u32 / 3,
                        pixel.1 * 3 + index as u32 % 3,
                    )
                })
                .collect::<Vec<_>>();

            grid_layout.extend(f_array);
        });

    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(width * 3, height * 3, Rgb([255, 255, 255]));

    grid_layout.iter().for_each(|(has_pixel, x, y)| {
        if !has_pixel {
            return;
        }

        let pixel = Rgb([0u8, 0u8, 0u8]);
        imgbuf.put_pixel(*x, *y, pixel);
    });
    imgbuf.save(target_path).unwrap();

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

pub fn run_tutorial(config: TutorialConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
