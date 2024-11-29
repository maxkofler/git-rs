use std::path::{Path, PathBuf};

use log::debug;

use crate::error::{ErrorExt, RepositoryError, Result};
use crate::str;
use crate::util::fs::{self};

use super::{Object, ObjectID};

/// A git repository
///
/// Note: This refers to the `.git` directory
pub struct Repository {
    /// The root where the repository lives in
    root: PathBuf,
}

impl Repository {
    /// Opens a repository (`.git`)
    /// # Arguments
    /// * `path` - The path to the repository
    ///
    /// This will not search for a repository, call [find_and_open](Self::find_and_open) instead.
    pub fn open(path: PathBuf) -> Result<Self> {
        let _self = Self { root: path };

        fs::create_dir_all(&_self.objects_dir())?;
        fs::create_dir_all(&_self.refs_heads_dir())?;

        Ok(_self)
    }

    /// Tries to find a repository in or upwards of `path` and opens it
    /// # Arguments
    /// * `path` - The path to search
    ///
    /// This will return an error if the repository is not found.
    /// To have another way of handling no repository, use [try_find_and_open](Self::try_find_and_open)
    pub fn find_and_open(path: &Path) -> Result<Self> {
        match Self::try_find_and_open(path)? {
            Some(repository) => Ok(repository),
            None => RepositoryError::NotFound.ctx(str!(
                "No upwards repository from {}",
                path.to_string_lossy()
            )),
        }
    }

    /// Tries to find a repository in or upwards of `path` and opens it
    /// # Arguments
    /// * `path` - The path to search
    pub fn try_find_and_open(path: &Path) -> Result<Option<Self>> {
        let mut cur_path = std::path::absolute(path)?;

        while let Some(parent) = cur_path.parent() {
            if std::fs::exists(cur_path.join(".git"))? {
                let repo = Self::open(cur_path.join(".git"))?;
                debug!("Found a repository @ {}", cur_path.to_string_lossy());
                return Ok(Some(repo));
            } else {
                cur_path = parent.to_owned();
            }
        }

        Ok(None)
    }

    /// Reads an object from this repository
    /// # Arguments
    /// * `oid` - The object id to read
    ///
    /// This function will error if the object is not found, use
    /// [try_read_object](Self::try_read_object) for a function that checks
    /// for object existence
    pub fn read_object(&self, oid: &ObjectID) -> Result<Object> {
        let path = self.objects_dir().join(oid.to_path());
        let mut file = fs::file_open(&path)?;

        Object::read(oid.clone(), &mut file)
    }

    /// Reads an object from this repository
    /// # Arguments
    /// * `oid` - The object id to read
    pub fn try_read_object(&self, oid: &ObjectID) -> Result<Option<Object>> {
        let path = self.objects_dir().join(oid.to_path());

        if !path.exists() {
            return Ok(None);
        } else {
            let mut file = fs::file_open(&path)?;

            Object::read(oid.clone(), &mut file).map(|o| Some(o))
        }
    }
}

impl Repository {
    /// Returns the subdirectory containing the object database
    pub fn objects_dir(&self) -> PathBuf {
        self.root.join("objects")
    }

    /// Returns the subdirectory containing the references
    pub fn refs_dir(&self) -> PathBuf {
        self.root.join("refs")
    }

    /// Returns the subdirectory containing head references
    pub fn refs_heads_dir(&self) -> PathBuf {
        self.refs_dir().join("heads")
    }
}
