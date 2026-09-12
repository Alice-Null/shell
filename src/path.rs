use std::path::PathBuf;
use crate::types::{Error, Operators};
/// basically just $PATH from shells
pub struct CmdPath {
    directories: Vec<PathBuf>
}

// wow this sucks so ba lmao
// skill issue whoever wrote this
// wait it was me
// oops

impl CmdPath {
    /// add a directory to search
    fn add_directory(&self, dir: PathBuf) -> Result<Ok(_), Err(Error)> {
        if dir.is_dir() {
            self.directories.push(dir);
        } else {
            Err(
                Error {
                    operator: Operators::
                }
            )
        }
    }
    fn {}
}