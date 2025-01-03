use std::process;

use book_pictures::cli::{CommandType, Config, ProcessTextConfig, TextCommandType};

fn main() {
    env_logger::init();

    log::debug!("Starting the application");

    let config: Config = argh::from_env();

    log::debug!("Parsed configuration: {:?}", config);

    match config.command {
        CommandType::Tutorial(tutorial_config) => {
            if let Err(e) = book_pictures::run_tutorial(tutorial_config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
        CommandType::ToBlackAndWhite(to_black_and_whit_config) => {
            if let Err(e) = book_pictures::run_to_black_and_white(to_black_and_whit_config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
        CommandType::GenerateGrid(generate_grid_config) => {
            if let Err(e) = book_pictures::run_generate_grid(generate_grid_config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
        CommandType::ProcessText(process_text_config) => {
            handle_process_text_subcommand(process_text_config)
        }
        // _ => println!("unknown command")
    }
}

fn handle_process_text_subcommand(config: ProcessTextConfig) {
    match config.text_command {
        TextCommandType::TextLength(text_length_config) => {
            if let Err(e) = book_pictures::run_text_length(text_length_config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
    }
    ()
}
