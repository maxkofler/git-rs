use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

/// A wrapper around an object ID (sha1)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ObjectID {
    hash: Vec<u8>,
}

impl Debug for ObjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectID")
            .field("hash", &self.to_string())
            .finish()
    }
}

impl ObjectID {
    /// Creates a new object id from a hash
    /// # Arguments
    /// * `hash` - The hash to take as a source
    pub fn new(hash: Vec<u8>) -> Self {
        Self { hash }
    }

    /// Decodes a object id from a hex string
    /// # Arguments
    /// * `hex_string` - The string to decode
    pub fn new_from_hex(hex_string: &str) -> Result<Self, hex::FromHexError> {
        let hash: Vec<u8> = hex::decode(hex_string)?;
        Ok(Self::new(hash))
    }

    /// Encodes this object id to a hex string
    pub fn to_hex_str(&self) -> String {
        hex::encode(&self.hash)
    }

    /// Returns the length of the object id in bytes
    pub fn len(&self) -> usize {
        self.hash.len()
    }

    /// Returns if the object id is empty
    pub fn is_empty(&self) -> bool {
        self.hash.is_empty()
    }

    /// Returns a byte slice of this object id
    pub fn bytes(&self) -> &[u8] {
        &self.hash
    }

    /// Converts this oid to a path consisting of the
    /// following parts: `aa/bbccdd...` to be used as the
    /// path in the object database
    pub fn to_path(&self) -> PathBuf {
        let string = self.to_hex_str();

        let tld = &string[0..2];
        let rest = &string[2..];

        PathBuf::from(tld).join(rest)
    }
}

impl Display for ObjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_str())
    }
}
