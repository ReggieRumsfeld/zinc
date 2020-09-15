//!
//! The bytecode binary file.
//!

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::file::error::Error;

///
/// The bytecode binary file representation.
///
pub struct Bytecode {
    /// The file contents.
    pub inner: Vec<u8>,
}

impl Bytecode {
    ///
    /// Creates a string with the default file name.
    ///
    fn file_name() -> String {
        zinc_const::file_name::BINARY.to_owned()
    }
}

impl TryFrom<&PathBuf> for Bytecode {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(Self::file_name()));
        }

        let mut file =
            File::open(path).map_err(|error| Error::Opening(Self::file_name(), error))?;
        let size = file
            .metadata()
            .map_err(|error| Error::Metadata(Self::file_name(), error))?
            .len() as usize;

        let mut buffer = Vec::with_capacity(size);
        file.read_to_end(&mut buffer)
            .map_err(|error| Error::Reading(Self::file_name(), error))?;

        Ok(Self { inner: buffer })
    }
}
