use crate::DisplayFormat;
use humansize::format_size;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DiskUsage {
    pub path: PathBuf,

    /// Size in bytes
    pub size: u64,

    pub is_dir: bool,
}

impl DiskUsage {
    pub fn format(&self, format: DisplayFormat) -> String {
        let size_str = match format {
            DisplayFormat::Bytes => format!("{}", self.size),
            DisplayFormat::Kilobytes => {
                // Round up to nearest kb
                format!("{}", (self.size + 1023).div_ceil(1024))
            }
            DisplayFormat::Binary => format_size(self.size, humansize::BINARY),
            DisplayFormat::Decimal => format_size(self.size, humansize::DECIMAL),
        };
        format!("{}\t{}", size_str, self.path.display())
    }
}
