use super::prelude::{Cache, DbKey, DbWriter};
use crate::model::stores::{errors::StoreError, DB};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// A concurrent DB store with typed caching.
#[derive(Clone)]
pub struct CachedDbAccess<TKey, TData>
where
    TKey: Clone + std::hash::Hash + Eq + Send + Sync,
    TData: Clone + Send + Sync,
{
    db: Arc<DB>,

    // Cache
    cache: Cache<TKey, Arc<TData>>,

    // DB bucket/path
    prefix: &'static [u8],
}

impl<TKey, TData> CachedDbAccess<TKey, TData>
where
    TKey: Clone + std::hash::Hash + Eq + Send + Sync,
    TData: Clone + Send + Sync,
{
    pub fn new(db: Arc<DB>, cache_size: u64, prefix: &'static [u8]) -> Self {
        Self { db, cache: Cache::new(cache_size), prefix }
    }

    pub fn read_from_cache(&self, key: TKey) -> Option<Arc<TData>>
    where
        TKey: Copy + AsRef<[u8]>,
    {
        self.cache.get(&key)
    }

    pub fn has(&self, key: TKey) -> Result<bool, StoreError>
    where
        TKey: Copy + AsRef<[u8]>,
    {
        Ok(self.cache.contains_key(&key) || self.db.get_pinned(DbKey::new(self.prefix, key))?.is_some())
    }

    pub fn read(&self, key: TKey) -> Result<Arc<TData>, StoreError>
    where
        TKey: Copy + AsRef<[u8]> + ToString,
        TData: DeserializeOwned, // We need `DeserializeOwned` since the slice coming from `db.get_pinned` has short lifetime
    {
        if let Some(data) = self.cache.get(&key) {
            Ok(data)
        } else {
            let db_key = DbKey::new(self.prefix, key);
            if let Some(slice) = self.db.get_pinned(&db_key)? {
                let data: Arc<TData> = Arc::new(bincode::deserialize(&slice)?);
                self.cache.insert(key, Arc::clone(&data));
                Ok(data)
            } else {
                Err(StoreError::KeyNotFound(db_key))
            }
        }
    }

    pub fn write(&self, mut writer: impl DbWriter, key: TKey, data: &Arc<TData>) -> Result<(), StoreError>
    where
        TKey: Copy + AsRef<[u8]>,
        TData: Serialize,
    {
        self.cache.insert(key, Arc::clone(data));
        let bin_data = bincode::serialize(data.as_ref())?;
        writer.put(DbKey::new(self.prefix, key), bin_data)?;
        Ok(())
    }

    pub fn write_many(
        &self,
        mut writer: impl DbWriter,
        iter: &mut (impl Iterator<Item = (TKey, Arc<TData>)> + Clone),
    ) -> Result<(), StoreError>
    where
        TKey: Copy + AsRef<[u8]>,
        TData: Serialize,
    {
        let iter_clone = iter.clone();
        self.cache.insert_many(iter);
        for (key, data) in iter_clone {
            let bin_data = bincode::serialize(data.as_ref())?;
            writer.put(DbKey::new(self.prefix, key), bin_data)?;
        }
        Ok(())
    }

    pub fn delete(&self, mut writer: impl DbWriter, key: TKey) -> Result<(), StoreError>
    where
        TKey: Copy + AsRef<[u8]>,
    {
        self.cache.remove(&key);
        writer.delete(DbKey::new(self.prefix, key))?;
        Ok(())
    }

    pub fn delete_many(&self, mut writer: impl DbWriter, key_iter: &mut (impl Iterator<Item = TKey> + Clone)) -> Result<(), StoreError>
    where
        TKey: Copy + AsRef<[u8]>,
    {
        let key_iter_clone = key_iter.clone();
        self.cache.remove_many(key_iter);
        for key in key_iter_clone {
            writer.delete(DbKey::new(self.prefix, key))?;
        }
        Ok(())
    }
}

/// A concurrent DB store with typed caching for `Copy` types.
/// TODO: try and generalize under `CachedDbAccess`
#[derive(Clone)]
pub struct CachedDbAccessForCopy<TKey, TData>
where
    TKey: Clone + std::hash::Hash + Eq + Send + Sync,
    TData: Clone + Copy + Send + Sync,
{
    db: Arc<DB>,

    // Cache
    cache: Cache<TKey, TData>,

    // DB bucket/path
    prefix: &'static [u8],
}

impl<TKey, TData> CachedDbAccessForCopy<TKey, TData>
where
    TKey: Clone + std::hash::Hash + Eq + Send + Sync,
    TData: Clone + Copy + Send + Sync,
{
    pub fn new(db: Arc<DB>, cache_size: u64, prefix: &'static [u8]) -> Self {
        Self { db, cache: Cache::new(cache_size), prefix }
    }

    pub fn has(&self, key: TKey) -> Result<bool, StoreError>
    where
        TKey: Copy + AsRef<[u8]>,
    {
        Ok(self.cache.contains_key(&key) || self.db.get_pinned(DbKey::new(self.prefix, key))?.is_some())
    }

    pub fn read(&self, key: TKey) -> Result<TData, StoreError>
    where
        TKey: Copy + AsRef<[u8]> + ToString,
        TData: DeserializeOwned, // We need `DeserializeOwned` since the slice coming from `db.get_pinned` has short lifetime
    {
        if let Some(data) = self.cache.get(&key) {
            Ok(data)
        } else {
            let db_key = DbKey::new(self.prefix, key);
            if let Some(slice) = self.db.get_pinned(&db_key)? {
                let data: TData = bincode::deserialize(&slice)?;
                self.cache.insert(key, data);
                Ok(data)
            } else {
                Err(StoreError::KeyNotFound(db_key))
            }
        }
    }

    pub fn write(&self, mut writer: impl DbWriter, key: TKey, data: TData) -> Result<(), StoreError>
    where
        TKey: Copy + AsRef<[u8]>,
        TData: Serialize,
    {
        self.cache.insert(key, data);
        let bin_data = bincode::serialize(&data)?;
        writer.put(DbKey::new(self.prefix, key), bin_data)?;
        Ok(())
    }
}