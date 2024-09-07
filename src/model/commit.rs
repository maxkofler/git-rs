use crate::{
    error::{CommitError, ErrorExt, ParsingError, Result},
    str,
    util::KVLM,
};

use super::ObjectID;

/// A commit
#[derive(Debug, Clone)]
pub struct Commit {
    /// The commit message attached to this commit
    pub message: String,
    /// The tree at this commit
    pub tree: ObjectID,
    /// The parent commit (if some)
    pub parent: Option<ObjectID>,
    /// The author that has written the code
    pub author: String,
    /// The committer
    pub committer: String,
    /// The GPG signature of the commit (if some)
    pub gpg_signature: Option<String>,
}

impl Commit {
    /// Parses a commit from the contents of an object
    /// # Arguments
    /// * `data` - The data to parse the commit from
    pub fn parse(data: Vec<u8>) -> Result<Self> {
        let string = String::from_utf8(data).ctx(str!("Parsing commit"))?;

        let kvlm = match KVLM::parse(&string) {
            Some(x) => x,
            None => return ParsingError::KVLM(string).ctx(|| "Parsing commit KVLM"),
        };

        Self::from_kvlm(kvlm)
    }

    /// Constructs a commit from a [KVLM]
    /// # Arguments
    /// * `kvlm` - The Key Value List with Message to parse
    pub fn from_kvlm(kvlm: KVLM) -> Result<Self> {
        let tree_str = match kvlm.map.get("tree") {
            Some(x) => x,
            None => return CommitError::MissingTree.ctx(|| "Parsing commit"),
        };
        let tree = ObjectID::new_from_hex(tree_str).ctx(|| "Parsing tree sha1")?;

        let author = match kvlm.map.get("author") {
            Some(x) => x.to_owned(),
            None => return CommitError::MissingAuthor.ctx(|| "Parsing commit"),
        };

        let committer = match kvlm.map.get("committer") {
            Some(x) => x.to_owned(),
            None => return CommitError::MissingCommitter.ctx(|| "Parsing commit"),
        };

        let gpg_signature = kvlm.map.get("gpgsig").map(|s| s.to_owned());
        let parent = if let Some(parent) = kvlm.map.get("parent") {
            match ObjectID::new_from_hex(parent) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        } else {
            None
        };

        Ok(Self {
            message: kvlm.message,
            tree,
            parent,
            author,
            committer,
            gpg_signature,
        })
    }
}
