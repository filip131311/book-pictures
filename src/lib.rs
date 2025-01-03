pub mod cli;

use cli::{GenerateGridConfig, ToBlackAndWhiteConfig, TutorialConfig};
use image::{GenericImageView, ImageBuffer, ImageReader, Rgb, Rgba};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fs;

pub fn run_to_black_and_white(config: ToBlackAndWhiteConfig) -> Result<(), Box<dyn Error>> {
    let source_path = &config.source_path;
    let target_path = &config
        .target_path
        .unwrap_or(String::from("black_and_white_img.png"));

    let img = ImageReader::open(source_path)?.decode()?;

    // let black_and_white_img = img.grayscale();
    let black_and_white_img = img.grayscale();
    // black_and_white_img.invert();
    black_and_white_img.save(target_path)?;

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
            let pixels_density =
                (((255 - pixel.2[0]) as f32 / 255f32 * pixel.2[3] as f32 / 255f32) * 9f32) as u8;

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
