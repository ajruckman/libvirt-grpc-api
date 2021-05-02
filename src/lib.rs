use std::convert::TryInto;
use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;

mod schema;

use tonic::Status;
use uuid::Uuid;
use std::collections::HashMap;

pub fn byte_vec_to_uuid(vec: Vec<u8>) -> Result<Uuid, Box<dyn error::Error>> {
    let bytes: [u8; 16] = vec.try_into().unwrap();
    let uuid = Uuid::from_bytes(bytes);

    return Ok(uuid);
}

pub fn enumerate_usb_devices() -> Result<Vec<schema::schema::USBDevice>, Box<dyn error::Error>> {
    let context = libudev::Context::new().unwrap();
    let mut enumerator = libudev::Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("usb").unwrap();

    let mut result: Vec<schema::schema::USBDevice> = vec![];

    for device in enumerator.scan_devices().unwrap() {
        let mut props: HashMap<String, String> = HashMap::new();

        for x in device.properties() {
            props.insert(
                x.name().to_string_lossy().to_string(),
                x.value().to_string_lossy().to_string(),
            );
        }

        if !props.contains_key("ID_MODEL_ID") || !props.contains_key("ID_VENDOR_ID") {
            continue;
        }

        let this = schema::schema::USBDevice {
            device: props["DEVNAME"].clone(),
            model: props["ID_MODEL"].clone(),
            vendor_id: props["ID_VENDOR_ID"].clone(),
            product_id: props["ID_MODEL_ID"].clone(),
            vendor_name: match props.contains_key("ID_VENDOR_FROM_DATABASE") {
                true => Some(props["ID_VENDOR_FROM_DATABASE"].clone()),
                false => None,
            },
            model_name: match props.contains_key("ID_MODEL_FROM_DATABASE") {
                true => Some(props["ID_MODEL_FROM_DATABASE"].clone()),
                false => None,
            },
        };

        println!("{}", this);

        result.push(this);
    }

    return Ok(result);
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
