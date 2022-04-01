mod command_service;

use std::sync::Arc;
use tracing::debug;
use crate::{CommandRequest, CommandResponse, KvError, MemTable, Storage};
use crate::command_request::RequestData;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            // TODO
            // inner: Arc::clone(&self.inner);
            inner: self.inner.clone(),
        }
    }
}

struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner {store})
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        let res = Self::dispatch(cmd, &self.inner.store);
        debug!("Executed response: {:?}", res);

        res
    }

    fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
        match cmd.request_data {
            Some(RequestData::Hget(param)) => param.execute(store),
            Some(RequestData::Hgetall(param)) => param.execute(store),
            Some(RequestData::Hset(param)) => param.execute(store),
            None => KvError::InvalidCommand("Request has no data".into()).into(),
            _ => KvError::Internal("Not implemented".into()).into(),
        }
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {

    }
}