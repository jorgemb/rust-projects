use std::{env, fs, io};
use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(test)]
mod tests;

/// Represents an error of the QueryManager
#[derive(Error, Debug)]
pub enum QueryManagerError {
    #[error("IO error")]
    IOError(#[from] io::Error),

    #[error("Couldn't initialize folder structure")]
    Initialize(String),
}


/// Allows to index, store, retrieve and validate queries from OpenAI
/// from the file manager
pub struct QueryManager {
    base_path: PathBuf,
    storage_paths: HashMap<&'static str, PathBuf>,
}

impl QueryManager {
    /// Expected directories to be created within the queries
    pub const STORAGE_DIRECTORIES: [&'static str; 2] = ["completions", "chats"];

    /// Base queries directory where everything else will be stored
    pub const QUERY_DIRECTORY: &'static str = "queries";

    /// Creates the default implementation using the CWD as base path
    pub fn build() -> Result<Self, QueryManagerError> {
        let mut current_dir = env::current_dir()?;
        current_dir.push(Self::QUERY_DIRECTORY);

        let base_path = current_dir.clone();

        // Create main directory
        if fs::create_dir_all(&base_path).is_err() {
            return Err(QueryManagerError::Initialize(format!("Failed to create directory {}", current_dir.display())));
        }

        // Create storage directories
        let mut storage_paths = HashMap::new();
        for directory in Self::STORAGE_DIRECTORIES {
            current_dir.push(directory);
            if fs::create_dir(&current_dir).is_err() {
                return Err(QueryManagerError::Initialize(format!("Failed to create storage directory: {}", current_dir.display())));
            }
            storage_paths.insert(directory, current_dir.clone());

            current_dir.pop();
        }

        Ok(QueryManager { base_path, storage_paths })
    }

    /// Returns a reference to the base path for all storage
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Returns a reference to the path of the given storage
    pub fn storage_paths(&self, storage: &str) -> Option<&PathBuf> {
        self.storage_paths.get(storage)
    }
}