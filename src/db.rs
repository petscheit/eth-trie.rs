use hashbrown::{HashMap};
use std::error::Error;
use sp_std::sync::Arc;

use parking_lot::RwLock;

// use crate::errors::MemDBError;

// use anyho

/// "DB" defines the "trait" of trie and database interaction.
/// You should first write the data to the cache and write the data
/// to the database in bulk after the end of a set of operations.
pub trait DB: Send + Sync {
    // type Error: Error;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, &'static str>; // need

    /// Insert data into the cache.
    fn insert(&self, key: &[u8], value: Vec<u8>) -> Result<(), &'static str>; // need
}

#[derive(Default, Debug)]
pub struct MemoryDB {
    // If "light" is true, the data is deleted from the database at the time of submission.
    light: bool,
    storage: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl MemoryDB {
    pub fn new(light: bool) -> Self {
        MemoryDB {
            light,
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl DB for MemoryDB {
    // type Error = MemDBError;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, &'static str> {
        if let Some(value) = self.storage.read().get(key) {
            Ok(Some(value.clone()))
        } else {
            Ok(None)
        }
    }

    fn insert(&self, key: &[u8], value: Vec<u8>) -> Result<(), &'static str> {
        self.storage.write().insert(key.to_vec(), value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memdb_get() {
        let memdb = MemoryDB::new(true);
        memdb.insert(b"test-key", b"test-value".to_vec()).unwrap();
        let v = memdb.get(b"test-key").unwrap().unwrap();

        assert_eq!(v, b"test-value")
    }

}
