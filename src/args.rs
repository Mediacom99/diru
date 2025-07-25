use clap::{Parser, ValueEnum};
use std::path::PathBuf;

//FIXME: remove from library, add agnostic config

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum DisplayFormat {
    /// Display sizes in bytes (raw)
    #[value(name = "bytes")]
    Bytes,

    /// Display sizes in kilobytes (1024-based) - default like GNU du
    #[value(name = "kb")]
    Kilobytes,

    /// Display sizes in human-readable binary format (KiB, MiB, GiB)
    #[value(name = "binary")]
    Binary,

    /// Display sizes in human-readable decimal format (KB, MB, GB)  
    #[value(name = "decimal")]
    #[default]
    Decimal,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Display format for sizes
    #[arg(
        short = 'f',
        long = "format",
        value_enum,
        default_value = "decimal",
        help = "Size display format"
    )]
    pub format: DisplayFormat,

    #[arg(long, short)]
    /// Show only a total for each argument
    pub summarize: bool,

    #[arg(long, short)]
    /// Show individual file sizes (not just directories)
    pub all: bool,

    #[arg(short = 'd', long = "max-depth", default_value = "999")]
    /// Maximum depth of directories to display
    pub max_depth: usize,

    #[arg(short = 'S', long = "follow-links")]
    /// Maximum depth of directories to display
    pub follow_links: bool,

    #[arg(short = 'l', long = "logical")]
    /// Show logical size instead of physical size
    pub logical: bool,
}
