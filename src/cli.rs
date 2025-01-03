use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Simulator server configuration.
pub struct Config {
    /// subcommand [process-picture, generate-grid, process-text]
    #[argh(subcommand)]
    pub command: CommandType,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum CommandType {
    ToBlackAndWhite(ToBlackAndWhiteConfig),
    GenerateGrid(GenerateGridConfig),
    Tutorial(TutorialConfig),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Generate a black and white picture based on the sourced picture.
#[argh(subcommand, name = "to_black_and_white")]
pub struct ToBlackAndWhiteConfig {
    /// the path to the source picture
    #[argh(positional)]
    pub source_path: String,
    #[argh(option)]
    /// the (optional) path for the new picture
    pub target_path: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Generate a grid picture mage of grey and white pixels based on an image.
#[argh(subcommand, name = "to_grid_image")]
pub struct GenerateGridConfig {
    /// the path to the source picture
    #[argh(positional)]
    pub source_path: String,
    #[argh(option)]
    /// the (optional) path for the new picture
    pub target_path: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// This is a subcommand form the tutorial https://doc.rust-lang.org/book/ch12-00-an-io-project.html
#[argh(subcommand, name = "tutorial")]
pub struct TutorialConfig {
    /// the searched string
    #[argh(positional)]
    pub query: String,

    /// the target file
    #[argh(positional)]
    pub file_path: String,

    /// should ignore case?
    #[argh(switch)]
    pub ignore_case: bool,
}
