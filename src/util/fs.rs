//! Filesystem utilities
use std::path::Path;

use log::trace;

use crate::error::{Error, ErrorExt};
use crate::str;

/// Creates a directory
///
/// Uses the [std::fs::create_dir()] function
pub fn create_dir(path: &Path) -> Result<(), Error> {
    trace!("Creating directory '{}'", path.to_string_lossy());
    std::fs::create_dir(path).ctx(str!("Creating directory '{}'", path.to_string_lossy()))
}

/// Creates a directory and all of its parents
///
/// Uses the [std::fs::create_dir_all()] function
pub fn create_dir_all(path: &Path) -> Result<(), Error> {
    trace!("Creating directory '{}'", path.to_string_lossy());
    std::fs::create_dir_all(path).ctx(str!("Creating directory '{}'", path.to_string_lossy()))
}

/// Creates a symlink pointing to `destination`
///
/// Uses the [std::os::unix::fs::symlink()] function
pub fn create_symlink(path: &Path, destination: &Path) -> Result<(), Error> {
    trace!(
        "Creating symlink '{}' pointing to '{}'",
        path.to_string_lossy(),
        destination.to_string_lossy()
    );

    // If the path exists, try to remove it first
    if path.exists() {
        std::fs::remove_file(path).ctx(str!(
            "Removing existing symlink or file {}",
            path.to_string_lossy()
        ))?
    }

    std::os::unix::fs::symlink(destination, path).ctx(|| {
        format!(
            "Creating symlink '{}' pointing to '{}'",
            path.to_string_lossy(),
            destination.to_string_lossy()
        )
    })
}

/// Copies `src` to `dest`
///
/// Uses the [std::fs::copy()] function
pub fn copy(src: &Path, dest: &Path) -> Result<u64, Error> {
    std::fs::copy(src, dest).ctx(|| {
        format!(
            "Copying '{}' to '{}'",
            src.to_string_lossy(),
            dest.to_string_lossy()
        )
    })
}

/// Renames `src` to `dest`
///
/// Uses the [std::fs::rename()] function
pub fn rename(src: &Path, dest: &Path) -> Result<(), Error> {
    std::fs::rename(src, dest).ctx(|| {
        format!(
            "Renaming '{}' to '{}'",
            src.to_string_lossy(),
            dest.to_string_lossy()
        )
    })
}

/// Remove a file
///
/// Uses the [std::fs::remove_file()] function
pub fn remove_file(path: &Path) -> Result<(), Error> {
    std::fs::remove_file(path).ctx(str!("Removing file '{}'", path.to_string_lossy()))
}

/// Remove an empty directory
///
/// Uses the [std::fs::remove_dir()] function
pub fn remove_dir(path: &Path) -> Result<(), Error> {
    std::fs::remove_dir(path).ctx(str!(
        "Removing empty directory '{}'",
        path.to_string_lossy()
    ))
}

/// Remove a directory and all of its contents
///
/// Uses the [std::fs::remove_dir_all()] function
pub fn remove_dir_all(path: &Path) -> Result<(), Error> {
    std::fs::remove_dir_all(path).ctx(str!(
        "Removing empty directory '{}'",
        path.to_string_lossy()
    ))
}

/// Opens a file using the [std::fs::File::open()] function
/// # Arguments
/// * `path` - The path to the file to open
pub fn file_open(path: &Path) -> Result<std::fs::File, Error> {
    std::fs::File::open(path).ctx(str!("Opening file {}", path.to_string_lossy()))
}

/// Creates a file using the [std::fs::File::create()] function
/// # Arguments
/// * `path` - The path to the file to create
pub fn file_create(path: &Path) -> Result<std::fs::File, Error> {
    std::fs::File::create(path).ctx(str!("Creating file {}", path.to_string_lossy()))
}
