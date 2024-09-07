/// A blob of data stored in the object database
#[derive(Debug, Clone)]
pub struct Blob {
    data: Vec<u8>,
}

impl Blob {
    /// Create a new blob that holds the supplied data
    /// # Arguments
    /// * `data` - The data to hold
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Returns a reference to the raw data contained
    /// by this blob
    pub fn raw_data(&self) -> &[u8] {
        &self.data
    }
}
