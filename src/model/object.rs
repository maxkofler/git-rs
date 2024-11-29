use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::{
    error::{Error, ErrorExt, ObjectError, ParsingError, Result},
    util::split::SplitOnceOwned,
};

use super::{Blob, Commit, ObjectID};

/// An object within git
#[derive(Debug, Clone)]
pub struct Object {
    /// The unique object id for this object
    pub oid: ObjectID,
    /// The type of object
    pub ty: ObjectType,
}

impl Object {
    /// Reads a git object by unpacking the contents
    /// of `read` as if it was coming directly from an
    /// object file
    /// # Arguments
    /// * `oid` - The object id to use for this object
    /// * `read` - The reader to read from
    pub fn read<R: Read>(oid: ObjectID, read: &mut R) -> Result<Self> {
        let mut data = Vec::new();

        let mut read = ZlibDecoder::new(read);

        read.read_to_end(&mut data)?;

        let ty = ObjectType::from_data(data)?;

        Ok(Self { oid, ty })
    }
}

/// The various types of objects
#[derive(Debug, Clone)]
pub enum ObjectType {
    /// A binary blob that wraps plain old data
    Blob(Blob),
    /// A commit
    Commit(Commit),
}

impl ObjectType {
    /// Parses an object type from binary data
    /// extracted from an object file or other
    /// things representing objects
    /// # Arguments
    /// * `data` - The data to interpret as an object
    pub fn from_data(data: Vec<u8>) -> Result<Self> {
        let (header, data) = match data.split_once_owned(0) {
            Some(x) => x,
            None => {
                return ParsingError::Split("Binary data".to_owned(), '\0')
                    .ctx(|| "Parsing object contents")
            }
        };

        let header_str = String::from_utf8(header).unwrap();
        let (obj_type, _obj_len) = match header_str.split_once(' ') {
            Some(x) => x,
            None => return ParsingError::Split(header_str, ' ').ctx(|| "Parsing object header"),
        };

        match obj_type {
            "blob" => Ok(Self::Blob(Blob::new(data))),
            "commit" => Ok(Self::Commit(Commit::parse(data)?)),
            ty => Err(Error::new(ObjectError::UnknownType(ty.to_owned()).into())),
        }
    }
}
