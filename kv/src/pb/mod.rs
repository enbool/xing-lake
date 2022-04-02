use http::StatusCode;
use crate::{CommandRequest, CommandResponse, Hget, Hgetall, Hset, KvError, Kvpair, Storage, Value};
use crate::command_request::RequestData;
use crate::service::CommandService;
use crate::value::Value::Integer;

pub mod abi;

impl CommandRequest {
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(
                Hget {
                    table: table.into(),
                    key: key.into(),
                }
            ))
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self{
            request_data: Some(RequestData::Hgetall(
                Hgetall {
                    table: table.into(),
                }
            ))
        }
    }

    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(
                Hset {
                    table: table.into(),
                    pair: Some(Kvpair::new(key, value)),
                }
            ))
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(data: (String, Value)) -> Self {
        Kvpair::new(data.0, data.1)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(crate::value::Value::String(s))
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(crate::value::Value::String(s.into()))
        }
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self {
            value: Some(Integer(i)),
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            value: vec![value],
            ..Default::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(pairs: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pair: pairs,
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(err: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: err.to_string(),
            value: vec![],
            pair: vec![],
        };

        match err {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }

        result
    }
}


/*impl CommandService for CommandRequest {
    fn execute(self, store: impl Storage) -> CommandResponse {
        if self.request_data.is_some() {
            return KvError::InvalidCommand("0".into()).into();
        }
        match self.request_data {
            Some(Hget) => {}
        }
    }
}
*/