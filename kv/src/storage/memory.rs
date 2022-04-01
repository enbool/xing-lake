use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use crate::{KvError, Kvpair, Storage, Value};

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
            },
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

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create(table);
        Ok(table.insert(key.into(), value))
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
            .collect()
        )
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item=Kvpair>>, KvError> {
        todo!()
    }
}