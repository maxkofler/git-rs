//! A `Key Value List with Message` used for commits and tags
use log::trace;
use ordermap::OrderMap;

/// A `Key Value List with Message` (`KVLM`) that
/// has a key-value store that is appended with a message
#[derive(Debug, Clone, Default)]
pub struct KVLM {
    /// The key-value store of the KVLM
    pub map: OrderMap<String, String>,
    /// The message that enriches this KVLM
    pub message: String,
}

impl KVLM {
    /// Parses a Key Value List with Message from a string
    /// # Arguments
    /// * `string` - The string to parse
    /// # Returns
    /// The KVLM or None if the parsing fails
    pub fn parse(string: &str) -> Option<Self> {
        let mut kvlm = Self::default();

        let mut buf = KVLMBuffer::default();

        let mut in_message = false;

        for line in string.lines() {
            if line.is_empty() {
                let (key, value) = buf.get();
                kvlm.map.insert(key.to_string(), value.to_string());
                buf.clear();
                in_message = true;
            }

            if !in_message {
                if let Some(stripped) = line.strip_prefix(' ') {
                    buf.add_line(stripped);
                } else {
                    let (key, value) = buf.get();
                    kvlm.map.insert(key.to_string(), value.to_string());

                    let (key, value) = line.split_once(' ')?;
                    buf.new_line(key.to_string(), value);
                }
            } else {
                buf.add_line(line);
            }
        }

        kvlm.message = buf.get().1.to_string();

        trace!("Parsed {:?} to {:?}", string, kvlm);

        Some(kvlm)
    }
}

/// A helper buffer function to store a key and
/// value while parsing multiple lines
#[derive(Default)]
struct KVLMBuffer {
    key: String,
    value: String,
}

impl KVLMBuffer {
    /// Starts a new line, clearing the buffer
    /// # Arguments
    /// * `key` - The key to insert
    /// * `value` - The value to insert
    pub fn new_line(&mut self, key: String, value: &str) {
        self.key = key;
        value.clone_into(&mut self.value);
    }

    /// Add a line to the `value` part of this buffer
    /// # Arguments
    /// * `value` - The value to append
    pub fn add_line(&mut self, value: &str) {
        if self.value.is_empty() {
            value.clone_into(&mut self.value);
        } else {
            self.value.push('\n');
            self.value += value;
        }
    }

    /// Returns the key and value as a tuple
    pub fn get(&self) -> (&str, &str) {
        (&self.key, &self.value)
    }

    /// Clears this buffer
    pub fn clear(&mut self) {
        self.key = String::new();
        self.value = String::new();
    }
}
