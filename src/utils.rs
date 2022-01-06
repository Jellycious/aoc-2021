/*
 * Various utility methods
 */

use std::env;
use std::io;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

/// Finds the root directory of the project.
pub fn get_project_path() -> io::Result<PathBuf> {
    let cur_dir = env::current_dir()?;
    let ancestors = cur_dir.ancestors();

    for ancestor in ancestors {
        if ancestor.join("Cargo.toml").exists() {
            // found root file
            return Ok(ancestor.to_path_buf());
        }
    };
    Err(io::Error::new(io::ErrorKind::NotFound, "Could not find project directory"))
}

//TODO: Refractor functions underneath, lots of reusable code
/// Finds the input directory
pub fn get_input_dir() -> io::Result<PathBuf> {
    let project_dir = get_project_path()?;
    let dir = project_dir.join("files");
    Ok(dir.join("inputs"))
}

#[allow(dead_code)]
/// Finds the description directory
pub fn get_description_dir() -> io::Result<PathBuf> {
    let project_dir = get_project_path()?;
    let dir = project_dir.join("files");
    Ok(dir.join("descriptions"))
}

/// Gets a filepath to an input file. Makes sure that the directories exist.
pub fn get_input_filepath(num: u32) -> io::Result<PathBuf> {
    assert!(num > 0 && num <= 25);
    let mut input_dir = get_input_dir()?;
    if !input_dir.exists() {
        // create input directory
        create_dir_all(&input_dir)?;
    }
    input_dir.push(Path::new(format!("day{}.txt", num).as_str()));
    Ok(input_dir)
}

#[allow(dead_code)]
/// Gets a filepath to an input file. Makes sure that the directories exist.
pub fn get_description_filepath(num: u32) -> io::Result<PathBuf> {
    assert!(num > 0 && num <= 25);
    let mut input_dir = get_description_dir()?;
    if !input_dir.exists() {
        // create input directory
        create_dir_all(&input_dir)?;
    }
    input_dir.push(Path::new(format!("day{}.md", num).as_str()));
    Ok(input_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// assumes test is ran from somewhere in the project directory.
    fn test_path_functions() {
        let project_path = get_project_path();
        assert!(project_path.is_ok());
        let project_path = project_path.ok().unwrap();
        assert!(project_path.join("Cargo.toml").exists());
        assert!(project_path.join("files").exists());
    }
}
