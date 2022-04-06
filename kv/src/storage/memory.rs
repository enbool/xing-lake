use crate::{KvError, Kvpair, Storage, StorageIter, Value};
use dashmap::mapref::one::Ref;
use dashmap::DashMap;

#[derive(Debug, Clone, Default)]
pub struct MemTable {
    table: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.table.get(name) {
            None => {
                let entry = self.table.entry(name.into()).or_default();
                entry.downgrade()
            }
            Some(table) => table,
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create(table);
        let res = table.get(key).map(|entry| entry.value().clone());
        Ok(res)
    }

    fn set(
        &self,
        table: &str,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create(table);
        Ok(table.insert(key.into(), value.into()))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.get_or_create(table);
        Ok(table.contains_key(key))
    }

    fn delete(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create(table);
        Ok(table.remove(key).map(|(_k, v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create(table);
        Ok(table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect())
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        let table = self.get_or_create(table).clone();

        // let iter = table.into_iter().map(|data| data.into());
        let mut iter = StorageIter::new(table.into_iter());
        let tmp = iter.next();

        Ok(Box::new(iter))
    }
}
