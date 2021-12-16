use std::{fs::read_to_string, io, path::PathBuf};

pub struct Loader {
    root: PathBuf,
}

impl Default for Loader {
    fn default() -> Self {
        let root = std::env::current_dir().unwrap().join("inputs");
        Self { root }
    }
}

impl Loader {
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Load the puzzle input from the given root directory
    ///
    /// This assumes that puzzle inputs are in plain-text files named "day1",
    /// "day2", etc.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the path doesn't exist
    pub fn load(&self, day: u16) -> io::Result<String> {
        let filename = format!("day{}", day);
        let path = self.root.join(&filename);
        read_to_string(path)
    }
}
