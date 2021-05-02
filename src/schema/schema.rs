use std::fmt;
use std::fmt::*;

use uuid::Uuid;

#[derive(Debug)]
pub struct Domain {
    pub uuid: uuid::Uuid,
    pub id: u32,
    pub name: String,
    pub hostname: Option<String>,
    pub os_type: Option<String>,
    pub state: DomainState,
    pub memory: u64,
    pub memory_max: u64,
    pub virt_cpu_num: u32,
    pub virt_cpu_time: u64,
}

#[repr(i32)]
pub enum DomainState {
    Unspecified = 0,
    NoState = 1,
    Running = 2,
    Blocked = 3,
    Paused = 4,
    ShutDown = 5,
    ShutOff = 6,
    Crashed = 7,
    PMSuspended = 8,
}

impl fmt::Debug for DomainState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Ok(())
    }
}

pub struct USBDevice {
    pub device: String,
    pub vendor_id: String,
    pub product_id: String,
    pub model: String,
    pub vendor_name: Option<String>,
    pub model_name: Option<String>,
}

impl fmt::Display for USBDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if !self.vendor_name.is_none() && !self.model_name.is_none() {
            write!(
                f,
                "{}: {}:{} {} ({} {})",
                self.device,
                self.vendor_id,
                self.product_id,
                self.model,
                self.vendor_name.as_ref().unwrap(),
                self.model_name.as_ref().unwrap()
            )
        } else {
            write!(
                f,
                "{}: {}:{} {}",
                self.device, self.vendor_id, self.product_id, self.model
            )
        }
    }
}
