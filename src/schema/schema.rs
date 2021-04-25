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
    Undefined = 0,
    NoState = 1,
    Running = 2,
    Blocked = 3,
    Paused = 4,
    ShutDown = 5,
    ShutOff = 6,
    Crashed = 7,
    PMSuspended = 8,
}
