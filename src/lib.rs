use uuid::Uuid;
use std::error;
use std::num::ParseIntError;
use std::fmt;
use std::convert::TryInto;

pub fn byte_vec_to_uuid(vec: Vec<u8>) -> Result<Uuid, Box<dyn error::Error>> {
    let bytes: [u8; 16] = vec.try_into().unwrap();
    let uuid = Uuid::from_bytes(bytes);

    return Ok(uuid);
}

#[derive(Debug)]
pub enum APIError {
    CreateDomain(GRPCErrorWithStatus),
    DestroyDomain(GRPCErrorWithStatus),
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::CreateDomain(x) => write!(f, "failed to create domain: {}", x.message),
            APIError::DestroyDomain(x) => write!(f, "failed to destroy domain: {}", x.message),
        }
    }
}

// impl Error for APIError {
//
// }

#[derive(Clone, Debug)]
pub struct GRPCErrorWithStatus {
    pub message: String,
    pub status: tonic::Status,
}

impl fmt::Display for GRPCErrorWithStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.status)
    }
}
