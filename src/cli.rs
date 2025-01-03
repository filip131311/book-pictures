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
    ProcessText(ProcessTextConfig),
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
