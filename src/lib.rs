
use std::fs;
use std::io;
use std::path::{Path, PathBuf};


/// If the search was unsuccessful.
#[derive(Debug)]
pub enum Error {
    /// Some std io Error occurred.
    IO(::std::io::Error),
    /// The directory requested was not found.
    NotFound,
}

impl ::std::convert::From<io::Error> for Error {
    fn from(io_err: io::Error) -> Error {
        Error::IO(io_err)
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        writeln!(f, "{:?}", *self)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref io_err) => ::std::error::Error::description(io_err),
            Error::NotFound => "The folder could not be found",
        }
    }
}


/// The direction in which `find_folder` should search for the folder.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Search {
    /// Search recursively through parent directories with the given depth.
    Parents(u8),
    /// Search recursively through children directories with the given depth.
    Kids(u8),
    /// Search in both directions (Parents first, then kids).
    Both(u8, u8),
}


impl Search {
    /// An easy API method for finding a folder with a given name.
    /// i.e. `Search::Kids(u8).for_folder("assets")`
    pub fn for_folder(&self, name: &str) -> Result<PathBuf, Error> {
        let cwd = try!(::std::env::current_dir());
        match *self {
            Search::Parents(depth) => check_parents(depth, name, &cwd),
            Search::Kids(depth) => check_kids(depth, name, &cwd),
            Search::Both(parents_depth, kids_depth) => {
                match check_parents(parents_depth, name, &cwd) {
                    Err(Error::NotFound) => check_kids(kids_depth, name, &cwd),
                    other_result => other_result,
                }
            },
        }
    }
}


/// Check the contents of this folder and children folders.
fn check_kids(depth: u8, name: &str, path: &Path) -> Result<PathBuf, Error> {
    match check_dir(name, path) {
        err @ Err(Error::NotFound) => match depth > 0 {
            true => {
                for entry in try!(fs::read_dir(path)) {
                    let entry = try!(entry);
                    let entry_path = entry.path();
                    if try!(fs::metadata(&entry_path)).is_dir() {
                        if let Ok(folder) = check_kids(depth-1, name, &entry_path) {
                            return Ok(folder);
                        }
                    }
                }
                err
            },
            false => err,
        },
        other_result => other_result,
    }
}

/// Check the given path and `depth` number of parent directories for a folder with the given name.
fn check_parents(depth: u8, name: &str, path: &Path) -> Result<PathBuf, Error> {
    match check_dir(name, path) {
        err @ Err(Error::NotFound) => match depth > 0 {
            true => match path.parent() {
                None => err,
                Some(parent) => check_parents(depth-1, name, parent),
            },
            false => err,
        },
        other_result => other_result,
    }
}

/// Check the given directory for a folder with the matching name.
fn check_dir(name: &str, path: &Path) -> Result<PathBuf, Error> {
    for entry in try!(fs::read_dir(path)) {
        let entry = try!(entry);
        let entry_path = entry.path();
        if entry_path.ends_with(name) {
            return Ok(entry_path)
        }
    }
    Err(Error::NotFound)
}

