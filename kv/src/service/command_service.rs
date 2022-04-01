use crate::{CommandResponse, CommandService, Hget, Hgetall, Hset, KvError, Storage, Value};

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(err) => err.into(),
        }
    }
}

impl CommandService  for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(err) => err.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => {
                match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                    Ok(Some(v)) => v.into(),
                    Ok(None) => Value::default().into(),
                    Err(err) => err.into(),
                }
            },
            None => KvError::InvalidCommand(format!("{:?}", self)).into()
        }
    }
}