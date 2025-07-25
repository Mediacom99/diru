use crate::{Args, DiskUsage};

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;
// use jwalk::WalkDir;

pub fn calculate_usage(path: &Path, cli_args: &Args) -> Result<Vec<DiskUsage>> {
    let mut results: Vec<DiskUsage> = Vec::new();

    if !path.exists() {
        anyhow::bail!("Path '{}' does not exist", path.display());
    }

    // Single file fast route
    if path.is_file() {
        let metadata = fs::metadata(path)
            .with_context(|| format!("Failed to read metadata for '{}'", path.display()))?;

        let file_size = if cli_args.logical {
            metadata.len()
        } else {
            metadata.blocks() * 512
        };

        results.push(DiskUsage {
            path: path.to_path_buf(),
            size: file_size,
            is_dir: false,
        });
        return Ok(results);
    }

    // Walks directory tree
    let walker = WalkDir::new(path)
        .follow_links(cli_args.follow_links)
        .max_depth(cli_args.max_depth);

    // Track directory sizes in hash map
    let mut dir_sizes: std::collections::HashMap<PathBuf, u64> = std::collections::HashMap::new();

    for entry in walker {
        let entry = entry.with_context(|| "Failed to read directory entry")?;

        let metadata = entry
            .metadata()
            .with_context(|| format!("Failed to read metadata for '{}'", entry.path().display()))?;

        //TODO Handle symlinks
        if !metadata.is_file() {
            continue;
        }

            let file_size = if cli_args.logical {
                metadata.len()
            } else {
                metadata.blocks() * 512
            };

            if cli_args.all && entry.depth() <= cli_args.max_depth {
                results.push(DiskUsage {
                    path: entry.path().to_path_buf(),
                    size: file_size,
                    is_dir: false,
                });
            }

            // For each file, add its size to ALL parent directories.
            // This gives us the total size for each directory in one pass.
            let mut current_path = entry.path();
            loop {
                *dir_sizes.entry(current_path.to_path_buf()).or_insert(0) += file_size;
                match current_path.parent() {
                    Some(parent) => {
                        *dir_sizes.entry(parent.to_path_buf()).or_insert(0) += file_size;
                        if parent == path {
                            break;
                        }
                        current_path = parent;
                    }
                    None => break,
                }
            }
        });

    // Show only directories up to max depth
    if !cli_args.summarize || cli_args.all {
        for (dir_path, size) in dir_sizes.iter() {
            //Skip root directory, add at the end
            if dir_path == path {
                continue;
            }

            let depth = dir_path
                .strip_prefix(path)
                .map(|p| p.components().count())
                .unwrap_or(0);

            if depth < cli_args.max_depth {
                results.push(DiskUsage {
                    path: dir_path.clone(),
                    size: *size,
                    is_dir: true,
                });
            }
        }
    }

    // Add the root dir level
    let total_size = dir_sizes.get(path).copied().unwrap_or(0);
    results.push(DiskUsage {
        path: path.to_path_buf(),
        size: total_size,
        is_dir: true,
    });

    //Sort results: directories first then by path
    results.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.path.cmp(&b.path),
    });

    Ok(results)
}
