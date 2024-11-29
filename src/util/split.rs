//! Utilities for splitting things

/// Trait for implementing split once functionality
pub trait SplitOnceOwned<T> {
    /// Splits `self` once at `delimiter`, returning a tuple
    /// # Arguments
    /// * `delimiter` - The delimiter to split at
    fn split_once_owned(self, delimiter: T) -> Option<(Self, Self)>
    where
        Self: Sized;
}

impl<T: PartialEq> SplitOnceOwned<T> for Vec<T> {
    fn split_once_owned(mut self, delimiter: T) -> Option<(Vec<T>, Vec<T>)> {
        if let Some(position) = self.iter().position(|x| *x == delimiter) {
            // Split the vector into two parts: first (up to the delimiter) and second (after the delimiter)
            let second = self.split_off(position + 1); // +1 to skip the delimiter
            self.pop(); // Remove the delimiter from the end of the first vector
            Some((self, second))
        } else {
            None
        }
    }
}

pub fn split_once(slice: &[u8], delimiter: u8) -> Option<(&[u8], &[u8])> {
    if let Some(index) = slice.iter().position(|&b| b == delimiter) {
        // Split at the index of the delimiter
        Some((&slice[..index], &slice[index + 1..]))
    } else {
        // Return None if delimiter is not found
        None
    }
}
