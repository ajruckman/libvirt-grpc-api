use std::convert::TryInto;
use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;

use tonic::Status;
use uuid::Uuid;

pub fn byte_vec_to_uuid(vec: Vec<u8>) -> Result<Uuid, Box<dyn error::Error>> {
    let bytes: [u8; 16] = vec.try_into().unwrap();
    let uuid = Uuid::from_bytes(bytes);

    return Ok(uuid);
}

pub struct GRPCAPIError {
    _msg: String,
    _status: Option<tonic::Status>,
}

impl<'a> GRPCAPIError {
    pub fn new(msg: String) -> GRPCAPIError {
        GRPCAPIError {
            _msg: msg,
            _status: None,
        }
    }

    fn new_with_status(msg: String, status: tonic::Status) -> GRPCAPIError {
        GRPCAPIError {
            _msg: msg,
            _status: Some(status),
        }
    }

    fn msg(&self) -> &String {
        &self._msg
    }

    fn status(&self) -> &Option<tonic::Status> {
        &self._status
    }
}

impl From<tonic::Status> for GRPCAPIError {
    fn from(v: Status) -> Self {
        GRPCAPIError::new_with_status(v.message().to_string(), v.clone())
    }
}

impl fmt::Debug for GRPCAPIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.status() {
            None => write!(f, "API error occurred: {}", self._msg),
            Some(v) => write!(f, "API error occurred: {} ({})", self._msg, v),
        }
    }
}
