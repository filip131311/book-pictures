use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Simulator server configuration.
pub struct Config {
    /// subcommand [to-black-and-white, generate-grid, process-text]
    #[argh(subcommand)]
    pub command: CommandType,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum CommandType {
    ToBlackAndWhite(ToBlackAndWhiteConfig),
    GenerateGrid(GenerateGridConfig),
    FindDistribution(FindDistributionConfig),
    ProcessText(ProcessTextConfig),
    CreateCustomImage(CreateCustomImageConfig),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Generate a black and white picture based on the sourced picture.
#[argh(subcommand, name = "to-black-and-white")]
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
#[argh(subcommand, name = "generate-grid")]
pub struct GenerateGridConfig {
    /// the path to the source picture
    #[argh(positional)]
    pub source_path: String,
    #[argh(option)]
    /// the (optional) path for the new picture
    pub target_path: Option<String>,
    #[argh(option, default = "3")]
    /// the (optional) size of generated grid (default is 3)
    pub grid_size: u32,
    #[argh(option, default = "1.0")]
    /// the (optional) gamma of generated grid (default is 1.0)
    pub gamma: f32,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Returns a gamma value that used for grid generation uses the closest possible number of pixels tio the provided text,
/// that still allows for fitting the whole text in the image.   
#[argh(subcommand, name = "find-distribution")]
pub struct FindDistributionConfig {
    /// the path to the source picture
    #[argh(positional)]
    pub img_source_path: String,
    /// the path to the source text
    #[argh(positional)]
    pub text_source_path: String,
    #[argh(option, default = "3")]
    /// the (optional) size of simulated grid (default is 3)
    pub grid_size: u32,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Generate a picture made out of provided text.
#[argh(subcommand, name = "create-custom-img")]
pub struct CreateCustomImageConfig {
    /// the path to the source picture
    #[argh(positional)]
    pub img_source_path: String,
    /// the path to the source text
    #[argh(positional)]
    pub text_source_path: String,
    #[argh(option)]
    /// the (optional) path for the created picture
    pub target_path: Option<String>,
    #[argh(option)]
    /// the size of generated grid
    pub grid_size: u32,
    #[argh(option)]
    /// the gamma of generated grid
    pub gamma: f32,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Pre-process txt file.
#[argh(subcommand, name = "process-text")]
pub struct ProcessTextConfig {
    /// the task to perform
    #[argh(subcommand)]
    pub text_command: TextCommandType,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum TextCommandType {
    TextLength(TextLengthConfig),
    ReplaceEnters(ReplaceEntersConfig),
    StripWhitespaces(StripWhitespacesConfig),
    RemoveMatchingLines(RemoveMatchingLinesConfig),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Return textfile length.
#[argh(subcommand, name = "length")]
pub struct TextLengthConfig {
    /// the path to the source text
    #[argh(positional)]
    pub source_path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Replace all enters with spaces.
#[argh(subcommand, name = "replace-enters")]
pub struct ReplaceEntersConfig {
    /// the path to the source text
    #[argh(positional)]
    pub source_path: String,
    #[argh(option)]
    /// the (optional) path for the new text file
    pub target_path: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Remove all white spaces from a file.
#[argh(subcommand, name = "strip-whitespaces")]
pub struct StripWhitespacesConfig {
    /// the path to the source text
    #[argh(positional)]
    pub source_path: String,
    #[argh(option)]
    /// the (optional) path for the new text file
    pub target_path: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Remove lines matching provided regex.
#[argh(subcommand, name = "remove-matching-lines")]
pub struct RemoveMatchingLinesConfig {
    /// the path to the source text
    #[argh(positional)]
    pub source_path: String,
    #[argh(positional)]
    /// regex for matching lines to be removed
    pub regex: String,
    #[argh(option)]
    /// the (optional) path for the new text file
    pub target_path: Option<String>,
}
